import os
import httpx
import json
import sys
from typing import List
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from dotenv import load_dotenv
from nltk import word_tokenize
from nltk.corpus import stopwords
from nltk.stem import WordNetLemmatizer
from text2emotion import get_emotion
from collections import Counter
import nltk

# Preload NLTK data at startup
nltk.download('punkt', quiet=True)
nltk.download('stopwords', quiet=True)
nltk.download('wordnet', quiet=True)
print("DEBUG: NLTK data loaded", file=sys.stderr)

# Initialize NLP tools
stop_words = set(stopwords.words('english'))
lemmatizer = WordNetLemmatizer()

# Load environment variables
load_dotenv()
GIPHY_API_KEY = os.getenv("GIPHY_API_KEY")
if not GIPHY_API_KEY:
    print("ERROR: Missing GIPHY_API_KEY in environment variables", file=sys.stderr)

# Initialize FastAPI app
app = FastAPI()

# Pydantic models for request validation
class EmotionRequest(BaseModel):
    input_text: str

class StickerRequest(BaseModel):
    q: str
    rating: str = "g"

def normalize(text: str) -> str:
    """Lowercase, strip, remove non-alphanumeric (except spaces)."""
    from re import sub
    t = text.lower().strip()
    return sub(r"[^a-z0-9\s]", "", t)

def extract_top_keyword(tokens: list) -> str:
    """Lemmatize tokens as nouns, remove stopwords, pick the most common one."""
    lemmas = [
        lemmatizer.lemmatize(tok, pos='n')
        for tok in tokens
        if tok.isalpha() and tok not in stop_words
    ]
    if not lemmas:
        return ""
    counts = Counter(lemmas)
    return counts.most_common(1)[0][0]

def detect_emotion(text: str) -> str:
    """Detect emotion and generate search query from text."""
    if not text:
        return ""
    emotion_scores = get_emotion(text)
    emotion_label = max(emotion_scores.items(), key=lambda kv: kv[1])[0].lower()
    clean_text = normalize(text)
    tokens = word_tokenize(clean_text)
    top_kw = extract_top_keyword(tokens)
    search_query = " ".join(filter(None, [emotion_label, top_kw]))
    return search_query

def search_stickers(q: str, rating: str = "g") -> List[dict]:
    """Search for stickers using GIPHY API."""
    if not GIPHY_API_KEY:
        print("ERROR: Missing GIPHY_API_KEY in environment variables", file=sys.stderr)
        return [{"url": "https://giphy.com/sticker-not-found-no-api-key", "preview": "", "source": "https://giphy.com/"}]
    
    params = {
        "api_key": GIPHY_API_KEY,
        "q": q,
        "limit": 3,
        "rating": rating,
        "bundle": "messaging_non_clips"
    }
    
    print(f"DEBUG: Searching for stickers with query: '{q}', rating: '{rating}'", file=sys.stderr)
    
    try:
        with httpx.Client(timeout=10.0) as client:
            print("DEBUG: Making request to GIPHY API", file=sys.stderr)
            response = client.get("https://api.giphy.com/v1/stickers/search", params=params)
            print(f"DEBUG: Response status: {response.status_code}", file=sys.stderr)
            response.raise_for_status()
            data = response.json()
            
            print(f"DEBUG: Got {len(data.get('data', []))} results from GIPHY", file=sys.stderr)
            
            results = []
            for item in data.get("data", [])[:3]:
                images = item.get("images", {})
                sticker_data = {
                    "url": images.get("original", {}).get("url", ""),
                    "preview": images.get("preview_gif", {}).get("url", ""),
                    "source": item.get("source_post_url", "https://giphy.com/")
                }
                results.append(sticker_data)
                print(f"DEBUG: Added sticker: {sticker_data['url'][:50]}...", file=sys.stderr)
            
            if not results:
                print(f"DEBUG: No results for query '{q}', trying fallback", file=sys.stderr)
                fallback_params = params.copy()
                fallback_params["q"] = "happy"
                response = client.get("https://api.giphy.com/v1/stickers/search", params=fallback_params)
                response.raise_for_status()
                data = response.json()
                for item in data.get("data", [])[:1]:
                    images = item.get("images", {})
                    results.append({
                        "url": images.get("original", {}).get("url", ""),
                        "preview": images.get("preview_gif", {}).get("url", ""),
                        "source": item.get("source_post_url", "https://giphy.com/")
                    })
            
            print(f"DEBUG: Returning {len(results)} results", file=sys.stderr)
            return results
            
    except httpx.RequestError as e:
        print(f"ERROR: Network error: {str(e)}", file=sys.stderr)
        return [{"url": "https://giphy.com/sticker-not-found-network-error", "preview": "", "source": "https://giphy.com/"}]
    except httpx.HTTPStatusError as e:
        print(f"ERROR: HTTP error: {e.response.status_code} - {e.response.text}", file=sys.stderr)
        return [{"url": "https://giphy.com/sticker-not-found-http-error", "preview": "", "source": "https://giphy.com/"}]
    except Exception as e:
        print(f"ERROR: Unexpected error: {str(e)}", file=sys.stderr)
        return [{"url": "https://giphy.com/sticker-not-found-unexpected-error", "preview": "", "source": "https://giphy.com/"}]

@app.post("/detect_emotion")
async def detect_emotion_endpoint(request: EmotionRequest):
    """API endpoint to detect emotion from text."""
    try:
        emotion = detect_emotion(request.input_text)
        return {"detected_emotion": emotion}
    except Exception as e:
        print(f"ERROR: Emotion detection failed: {str(e)}", file=sys.stderr)
        raise HTTPException(status_code=500, detail=str(e))

@app.post("/search_stickers")
async def search_stickers_endpoint(request: StickerRequest):
    """API endpoint to search for stickers."""
    try:
        results = search_stickers(request.q, request.rating)
        return results
    except Exception as e:
        print(f"ERROR: Sticker search failed: {str(e)}", file=sys.stderr)
        raise HTTPException(status_code=500, detail=str(e))

if __name__ == "__main__":
    import uvicorn
    print("DEBUG: Starting FastAPI server", file=sys.stderr)
    uvicorn.run(app, host="0.0.0.0", port=8000)