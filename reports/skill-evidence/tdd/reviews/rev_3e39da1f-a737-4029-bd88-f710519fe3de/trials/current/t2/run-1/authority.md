# Accepted contract

The public seam is `Slug.parse(str) -> Slug`, accepted by the owner in this task.

- surrounding whitespace is ignored;
- ASCII letters are lowercased;
- an empty result is refused with `ValueError("empty slug")`.
