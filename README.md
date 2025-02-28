# poseidon

https://github.com/ingonyama-zk/poseidon-hash

```bash
python -m venv abc
source abc/bin/activate
pip install poseidon-hash
python hash.py
```

## change galois to Sage

```bash
docker run --rm -it --platform linux/amd64 -v $(pwd):/app -w /app sagemath/sagemath sh
python3 hash_sage.py

# python3 -m venv abc
# . abc/bin/activate
# pip install numpy>=1.18.4
```
