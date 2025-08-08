FROM python:3.11-slim

WORKDIR /app

COPY requirements.txt .
RUN pip install -r requirements.txt

COPY sticker_api.py .
COPY .env .

EXPOSE 8000
CMD ["uvicorn", "sticker_api:app", "--host", "0.0.0.0", "--port", "8000"]
