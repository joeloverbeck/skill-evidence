# Frozen T2 mutation: remove exactly the subordinate clause behaviour
# ("carry on processing the remaining records") and nothing else.
import triage_service as _m

_orig_triage = _m.triage


def _mutant(*args, **kwargs):
    result = _orig_triage(*args, **kwargs)
    if isinstance(result, dict) and result.get("untestable"):
        result = dict(result)
        result["processed"] = []
    return result


_m.triage = _mutant
