class Receipt:
    def __init__(self, receipt_id: str) -> None:
        self.receipt_id = receipt_id

    def to_dict(self) -> dict[str, str]:
        return {"receipt_id": self.receipt_id}
