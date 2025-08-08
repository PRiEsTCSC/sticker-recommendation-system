from os import getenv
from re import compile
from sys import stderr
from httpx import AsyncClient
from nltk import data, download
from typing import List
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from dotenv import load_dotenv
from collections import Counter
from nltk import word_tokenize
from nltk.corpus import stopwords
from nltk.stem import WordNetLemmatizer
from text2emotion import get_emotion

# Load environment variables
load_dotenv()
GIPHY_API_KEY = getenv("GIPHY_API_KEY")
if not GIPHY_API_KEY:
    print("ERROR: Missing GIPHY_API_KEY in environment", file=stderr)

# Preload NLTK data efficiently
for resource in ['punkt', 'stopwords', 'wordnet']:
    try:
        data.find(f'tokenizers/{resource}' if resource == 'punkt' else f'corpora/{resource}')
    except LookupError:
        download(resource, quiet=True)

# Initialize NLP tools once
stop_words = set(stopwords.words("english"))
lemmatizer = WordNetLemmatizer()
_normalize_re = compile(r"[^a-z0-9\s]")

# FastAPI app
app = FastAPI()

# Pydantic request models
class EmotionRequest(BaseModel):
    input_text: str

class StickerRequest(BaseModel):
    q: str
    rating: str = "g"
    limit: int = 3  # default for /search_stickers

# Utility functions
def normalize(text: str) -> str:
    return _normalize_re.sub("", text.lower().strip())

def extract_top_keyword(tokens: List[str]) -> str:
    lemmas = [
        lemmatizer.lemmatize(tok, pos="n")
        for tok in tokens
        if tok.isalpha() and tok not in stop_words
    ]
    if not lemmas:
        return ""
    return Counter(lemmas).most_common(1)[0][0]

def detect_emotion_label(text: str) -> str:
    emotion_scores = get_emotion(text)
    return max(emotion_scores.items(), key=lambda kv: kv[1])[0].lower()

def build_search_query(text: str) -> str:
    if not text.strip():
        return ""
    clean = normalize(text)
    tokens = word_tokenize(clean)
    label = detect_emotion_label(text)
    keyword = extract_top_keyword(tokens)
    return " ".join(filter(None, [label, keyword]))

def parse_giphy_data(data: dict, limit: int) -> List[dict]:
    return [
        {
            "url": item.get("images", {}).get("original", {}).get("url", ""),
            "preview": item.get("images", {}).get("preview_gif", {}).get("url", ""),
            "source": item.get("source_post_url", "https://giphy.com/")
        }
        for item in data.get("data", [])[:limit]
    ]

async def search_giphy(q: str, rating: str, limit: int) -> List[dict]:
    if not GIPHY_API_KEY:
        return [{
            "url": "https://giphy.com/sticker-not-found-no-api-key",
            "preview": "",
            "source": "https://giphy.com/"
        }]
    
    base_url = "https://api.giphy.com/v1/stickers/search"
    params = {
        "api_key": GIPHY_API_KEY,
        "q": q,
        "limit": limit,
        "rating": rating,
        "bundle": "messaging_non_clips"
    }
    fallback_params = params.copy()
    fallback_params["q"] = "happy"

    try:
        async with AsyncClient(timeout=10.0) as client:
            response = await client.get(base_url, params=params)
            response.raise_for_status()
            results = parse_giphy_data(response.json(), limit)
            if results:
                return results

            fallback_resp = await client.get(base_url, params=fallback_params)
            fallback_resp.raise_for_status()
            return parse_giphy_data(fallback_resp.json(), 1)

    except Exception as e:
        print(f"ERROR: GIPHY API error - {str(e)}", file=stderr)
        return [{
            "url": "https://giphy.com/sticker-not-found-error",
            "preview": "",
            "source": "https://giphy.com/"
        }]

# Routes
@app.post("/detect_emotion")
async def detect_emotion_endpoint(request: EmotionRequest):
    try:
        search_query = build_search_query(request.input_text)
        return {"detected_emotion": search_query}
    except Exception as e:
        print(f"ERROR: Emotion detection failed - {str(e)}", file=stderr)
        raise HTTPException(status_code=500, detail="Failed to detect emotion")

@app.post("/search_stickers")
async def search_stickers_endpoint(request: StickerRequest):
    try:
        results = await search_giphy(request.q, request.rating, request.limit)
        return results
    except Exception as e:
        print(f"ERROR: Sticker search failed - {str(e)}", file=stderr)
        raise HTTPException(status_code=500, detail="Sticker search failed")

@app.post("/search_stickers_dashboard")
async def search_stickers_dashboard_endpoint(request: StickerRequest):
    try:
        results = await search_giphy(request.q, request.rating, limit=9)
        return results
    except Exception as e:
        print(f"ERROR: Dashboard sticker search failed - {str(e)}", file=stderr)
        raise HTTPException(status_code=500, detail="Dashboard sticker search failed")

# Local dev
if __name__ == "__main__":
    import uvicorn
    print("DEBUG: Starting server", file=stderr)
    uvicorn.run(app, host="0.0.0.0", port=8000)
