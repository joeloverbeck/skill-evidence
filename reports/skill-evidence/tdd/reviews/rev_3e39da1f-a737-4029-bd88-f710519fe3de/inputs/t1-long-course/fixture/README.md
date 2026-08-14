# Routebook fixture

Run the baseline suite with:

```bash
python3 -m unittest discover -s tests -v
```

The package exposes only the names in `routebook.__all__`. Tests exercise those public functions.
