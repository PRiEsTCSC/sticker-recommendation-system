# from typing import List
# import httpx
# import os
# from dotenv import load_dotenv

# # Load environment variables
# load_dotenv()

# # Get API key
# GIPHY_API_KEY = os.getenv("GIPHY_API_KEY")
# if not GIPHY_API_KEY:
#     raise RuntimeError("Missing GIPHY_API_KEY in environment variables")

# def search_stickers(q: str, rating: str = "g") -> List[dict]:
#     """
#     Search for stickers using GIPHY API.
    
#     Args:
#         q (str): Search query
#         rating (str): Content rating (g, pg, pg-13, r)
    
#     Returns:
#         List[dict]: List of sticker data with url, preview and source
#     """
    
#     # Build GIPHY API parameters
#     params = {
#         "api_key": GIPHY_API_KEY,
#         "q": q,
#         "limit": 3,
#         "rating": rating,
#         "bundle": "messaging_non_clips"
#     }
    
#     try:
#         with httpx.Client(timeout=10.0) as client:
#             response = client.get(
#                 "https://api.giphy.com/v1/stickers/search",
#                 params=params
#             )
#             response.raise_for_status()
#             data = response.json()
            
#             # Extract essential data for up to 3 stickers
#             results = []
#             for item in data.get("data", [])[:3]:
#                 images = item.get("images", {})
#                 results.append({
#                     "url": images.get("original", {}).get("url", ""),
#                     "preview": images.get("preview_gif", {}).get("url", ""),
#                     "source": item.get("source_post_url", "https://giphy.com/")
#                 })
                
#             # If no results, try original query as fallback
#             if not results:
#                 params["q"] = q
#                 response = client.get(
#                     "https://api.giphy.com/v1/stickers/search",
#                     params=params
#                 )
#                 response.raise_for_status()
#                 data = response.json()
#                 for item in data.get("data", [])[:3]:
#                     images = item.get("images", {})
#                     results.append({
#                         "url": images.get("original", {}).get("url", ""),
#                         "preview": images.get("preview_gif", {}).get("url", ""),
#                         "source": item.get("source_post_url", "https://giphy.com/")
#                     })
                    
#             return results
            
#     except httpx.HTTPStatusError as e:
#         error_msg = f"GIPHY API error: {e.response.status_code}"
#         if e.response.status_code == 403:
#             error_msg += " - Invalid API key"
#         elif e.response.status_code == 429:
#             error_msg += " - Rate limit exceeded"
#         raise RuntimeError(error_msg)
        
#     except httpx.RequestError as e:
#         raise RuntimeError(f"Network error: {str(e)}")
    





# if __name__ == "__main__":
#     import sys
#     import json
    
#     try:
#         input_data = json.loads(sys.stdin.read())
#         results = search_stickers(input_data["q"], input_data["rating"])
#         print(json.dumps(results))
#         sys.exit(0)
#     except Exception as e:
#         print(json.dumps({"error": str(e)}), file=sys.stderr)
#         sys.exit(1)

# import os
# import httpx
# import json
# from typing import List
# from dotenv import load_dotenv

# # Load environment variables
# load_dotenv()

# # Get API key
# GIPHY_API_KEY = os.getenv("GIPHY_API_KEY")
# if not GIPHY_API_KEY:
#     raise RuntimeError("Missing GIPHY_API_KEY in environment variables")

# def search_stickers(q: str, rating: str = "g") -> List[dict]:
#     """
#     Search for stickers using GIPHY API.
    
#     Args:
#         q (str): Search query
#         rating (str): Content rating (g, pg, pg-13, r)
    
#     Returns:
#         List[dict]: List of sticker data with url, preview and source
#     """
    
#     # Build GIPHY API parameters
#     params = {
#         "api_key": GIPHY_API_KEY,
#         "q": q,
#         "limit": 3,
#         "rating": rating,
#         "bundle": "messaging_non_clips"
#     }
    
#     try:
#         with httpx.Client(timeout=10.0) as client:
#             response = client.get("https://api.giphy.com/v1/stickers/search", params=params)
#             response.raise_for_status()
#             data = response.json()
            
#             results = []
#             for item in data.get("data", [])[:3]:
#                 images = item.get("images", {})
#                 results.append({
#                     "url": images.get("original", {}).get("url", ""),
#                     "preview": images.get("preview_gif", {}).get("url", ""),
#                     "source": item.get("source_post_url", "https://giphy.com/")
#                 })
            
#             # If no results, try fallback query
#             if not results:
#                 response = client.get("https://api.giphy.com/v1/stickers/search", params=params)
#                 response.raise_for_status()
#                 data = response.json()
#                 for item in data.get("data", [])[:3]:
#                     images = item.get("images", {})
#                     results.append({
#                         "url": images.get("original", {}).get("url", ""),
#                         "preview": images.get("preview_gif", {}).get("url", ""),
#                         "source": item.get("source_post_url", "https://giphy.com/")
#                     })
#             return results
            
#     except httpx.RequestError as e:
#         # Fallback: Return a default sticker result
#         return [{
#             "url": "https://giphy.com/sticker-not-found",
#             "preview": "",
#             "source": "https://giphy.com/"
#         }]
#     except httpx.HTTPStatusError as e:
#         error_msg = f"GIPHY API error: {e.response.status_code}"
#         raise RuntimeError(error_msg)

# if __name__ == "__main__":
#     import sys
#     try:
#         input_data = json.loads(sys.stdin.read())
#         results = search_stickers(input_data["q"], input_data["rating"])
#         print(json.dumps(results))
#         sys.exit(0)
#     except Exception as e:
#         print(json.dumps({"error": str(e)}), file=sys.stderr)
#         sys.exit(1)

import os
import httpx
import json
import sys
from typing import List
from dotenv import load_dotenv

# Load environment variables
load_dotenv()

# Get API key
GIPHY_API_KEY = os.getenv("GIPHY_API_KEY")

def search_stickers(q: str, rating: str = "g") -> List[dict]:
    """
    Search for stickers using GIPHY API.
    
    Args:
        q (str): Search query
        rating (str): Content rating (g, pg, pg-13, r)
    
    Returns:
        List[dict]: List of sticker data with url, preview and source
    """
    
    # Check if API key exists
    if not GIPHY_API_KEY:
        print(f"ERROR: Missing GIPHY_API_KEY in environment variables", file=sys.stderr)
        return [{
            "url": "https://giphy.com/sticker-not-found-no-api-key",
            "preview": "",
            "source": "https://giphy.com/"
        }]
    
    # Build GIPHY API parameters
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
            print(f"DEBUG: Making request to GIPHY API", file=sys.stderr)
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
                print(f"DEBUG: No results found for query '{q}', trying fallback", file=sys.stderr)
                # Try with a more generic query
                fallback_params = params.copy()
                fallback_params["q"] = "happy"
                response = client.get("https://api.giphy.com/v1/stickers/search", params=fallback_params)
                response.raise_for_status()
                data = response.json()
                
                for item in data.get("data", [])[:1]:  # Just get one fallback
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
        return [{
            "url": "https://giphy.com/sticker-not-found-network-error",
            "preview": "",
            "source": "https://giphy.com/"
        }]
    except httpx.HTTPStatusError as e:
        print(f"ERROR: HTTP error: {e.response.status_code} - {e.response.text}", file=sys.stderr)
        return [{
            "url": "https://giphy.com/sticker-not-found-http-error",
            "preview": "",
            "source": "https://giphy.com/"
        }]
    except Exception as e:
        print(f"ERROR: Unexpected error: {str(e)}", file=sys.stderr)
        return [{
            "url": f"https://giphy.com/sticker-not-found-unexpected-error",
            "preview": "",
            "source": "https://giphy.com/"
        }]

if __name__ == "__main__":
    try:
        print("DEBUG: Python script started", file=sys.stderr)
        input_data = json.loads(sys.stdin.read())
        print(f"DEBUG: Input data: {input_data}", file=sys.stderr)
        
        results = search_stickers(input_data["q"], input_data["rating"])
        print(json.dumps(results))
        sys.exit(0)
    except json.JSONDecodeError as e:
        print(f"ERROR: Invalid JSON input: {str(e)}", file=sys.stderr)
        print(json.dumps([{"url": "https://giphy.com/sticker-not-found-json-error", "preview": "", "source": "https://giphy.com/"}]))
        sys.exit(1)
    except KeyError as e:
        print(f"ERROR: Missing required key: {str(e)}", file=sys.stderr)
        print(json.dumps([{"url": "https://giphy.com/sticker-not-found-key-error", "preview": "", "source": "https://giphy.com/"}]))
        sys.exit(1)
    except Exception as e:
        print(f"ERROR: Script error: {str(e)}", file=sys.stderr)
        print(json.dumps([{"url": "https://giphy.com/sticker-not-found-script-error", "preview": "", "source": "https://giphy.com/"}]))
        sys.exit(1)