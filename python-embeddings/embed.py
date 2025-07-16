from sentence_transformers import SentenceTransformer

_model = None


def get_model():
    global _model
    if _model is None:
        _model = SentenceTransformer("all-MiniLM-L6-v2")
    return _model


def embed(text):
    model = get_model()
    vec = model.encode([text])[0]  # returns numpy array
    return vec.tolist()  # so it's JSON serializable
