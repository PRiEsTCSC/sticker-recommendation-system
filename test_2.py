from re import sub
from nltk import word_tokenize 
from nltk.corpus import stopwords
from nltk.stem import WordNetLemmatizer # type: ignore
from  text2emotion import get_emotion as te 
from collections import Counter

# Download required NLTK data (uncomment on first run)
# nltk.download('punkt', quiet=True)
# nltk.download('stopwords', quiet=True)
# nltk.download('wordnet', quiet=True)
# nltk.download('vader_lexicon', quiet=True)

# Initialize NLP tools
stop_words = set(stopwords.words('english'))
lemmatizer = WordNetLemmatizer()

def normalize(text: str) -> str:
    """Lowercase, strip, remove non-alphanumeric (except spaces)."""
    t = text.lower().strip()
    return sub(r"[^a-z0-9\s]", "", t)

def extract_top_keyword(tokens: list) -> str:
    """
    Lemmatize tokens as nouns, remove stopwords,
    then pick the single most common one.
    """
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
    """
    Main function to detect emotion and generate search query from text.
    
    Args:
        text (str): Input text to analyze
        
    Returns:
        str: Search query string
    """
    if not text:
        return ""
    
    # Sentiment analysis (VADER scores)
    # sentiment_scores = sia.polarity_scores(text)
    
    # Emotion label via text2emotion
    emotion_scores = te(text)
    emotion_label = max(emotion_scores.items(), key=lambda kv: kv[1])[0].lower() # type: ignore
    
    # Tokenize & extract top keyword
    clean_text = normalize(text)
    tokens = word_tokenize(clean_text)
    top_kw = extract_top_keyword(tokens)
    
    # Build the final search query
    search_query = " ".join(filter(None, [emotion_label, top_kw]))
    
    return search_query


# ...existing code...

if __name__ == "__main__":
    import sys
    import json
    
    try:
        input_data = json.loads(sys.stdin.read())
        text = input_data["input_text"]
        emotion = detect_emotion(text)
        print(json.dumps({"detected_emotion": emotion}))
        sys.exit(0)
    except Exception as e:
        print(json.dumps({"error": str(e)}), file=sys.stderr)
        sys.exit(1)