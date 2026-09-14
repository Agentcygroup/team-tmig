#!/usr/bin/env python3
"""Read ~/.tmig/hitl.jsonl and emit a mastery map.

Four queries:
  specialization — role x consequence class, verified/proposed ratio
  trajectory     — role x week, proposed and verified counts
  coverage       — role, classes touched / mastered / untouched
  transfer       — role pairs, Jaccard similarity over class coverage
"""
import json, pathlib, hashlib, sys, datetime
from collections import defaultdict, Counter

LEDGER = pathlib.Path.home() / ".tmig" / "hitl.jsonl"
OUT    = pathlib.Path.home() / "team-tmig" / "target" / "provenance" / "mastery-map.json"

def load_events():
    if not LEDGER.exists():
        return []
    out = []
    for line in LEDGER.read_text().splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            out.append(json.loads(line))
        except Exception:
            pass
    return out

def week_of(ts):
    try:
        dt = datetime.datetime.fromisoformat(ts.replace("Z", "+00:00"))
        iso = dt.isocalendar()
        return f"{iso[0]}-W{iso[1]:02d}"
    except Exception:
        return "unknown"

def consequence_of(e):
    return (e.get("proposal") or {}).get("consequence") or "Unknown"

def main():
    events = load_events()

    proposed = {}     # id -> {consequence, week}
    approved = set()
    executed = set()
    verified = set()
    revoked  = set()

    for e in events:
        t = e.get("type", "")
        if t == "Proposed":
            p = e.get("proposal") or {}
            pid = p.get("id")
            if pid:
                proposed[pid] = {
                    "consequence": p.get("consequence", "Unknown"),
                    "week": week_of(e.get("ts", "")),
                }
        elif t == "Approved":
            a = e.get("approval") or {}
            pid = a.get("proposal_id") or e.get("proposal_id")
            role = a.get("role")
            if pid and role:
                approved.add((pid, role))
        elif t == "Executed":
            ex = e.get("execution") or {}
            pid = ex.get("proposal_id")
            if pid:
                executed.add(pid)
        elif t == "Verified":
            v = e.get("verification") or {}
            pid = v.get("proposal_id")
            if pid:
                verified.add(pid)
        elif t == "Revoked":
            rid = e.get("revoked_event_hash")
            if rid:
                revoked.add(rid)

    # role -> class -> {proposed, verified}
    spec = defaultdict(lambda: defaultdict(lambda: {"proposed": 0, "verified": 0}))

    # approvals carry role; for each approval, credit the role with
    # the proposal's class, and count verified if the proposal was verified.
    for (pid, role) in approved:
        meta = proposed.get(pid, {})
        c = meta.get("consequence", "Unknown")
        spec[role][c]["proposed"] += 1
        if pid in verified:
            spec[role][c]["verified"] += 1

    specialization = []
    for role in sorted(spec):
        for c in sorted(spec[role]):
            d = spec[role][c]
            ratio = d["verified"] / d["proposed"] if d["proposed"] else 0.0
            specialization.append({
                "role": role,
                "consequence_class": c,
                "proposed": d["proposed"],
                "verified": d["verified"],
                "ratio": round(ratio, 4),
            })

    # trajectory: role x week
    traj = defaultdict(lambda: {"proposed": 0, "verified": 0})
    for (pid, role) in approved:
        meta = proposed.get(pid)
        if not meta:
            continue
        wk = meta.get("week", "unknown")
        traj[(role, wk)]["proposed"] += 1
        if pid in verified:
            traj[(role, wk)]["verified"] += 1

    trajectory = []
    for (role, wk) in sorted(traj):
        d = traj[(role, wk)]
        ratio = d["verified"] / d["proposed"] if d["proposed"] else 0.0
        trajectory.append({
            "role": role,
            "week": wk,
            "proposed": d["proposed"],
            "verified": d["verified"],
            "ratio": round(ratio, 4),
        })

    # coverage
    all_classes = sorted({v["consequence"] for v in proposed.values()} | {"None"})
    coverage = []
    for role in sorted(spec):
        touched   = set(spec[role].keys())
        mastered  = {c for c, d in spec[role].items()
                     if d["proposed"] > 0 and d["verified"] == d["proposed"]}
        untouched = set(all_classes) - touched
        coverage.append({
            "role": role,
            "classes_touched":   sorted(touched),
            "classes_mastered":  sorted(mastered),
            "classes_untouched": sorted(untouched),
            "coverage_ratio":    round(len(touched) / len(all_classes), 4) if all_classes else 0.0,
        })

    # transfer: Jaccard of touched classes per role pair
    roles = sorted(spec)
    transfer = []
    for i, a in enumerate(roles):
        for b in roles[i+1:]:
            ta = set(spec[a].keys())
            tb = set(spec[b].keys())
            inter = len(ta & tb)
            union = len(ta | tb)
            j = inter / union if union else 0.0
            transfer.append({
                "a": a, "b": b,
                "overlap_classes": inter,
                "jaccard": round(j, 4),
            })

    corpus_hash = hashlib.sha256(LEDGER.read_bytes()).hexdigest() if LEDGER.exists() else "missing"

    manifest = {
        "schema": "tmig/mastery-map/v1",
        "generated_at": datetime.datetime.utcnow().strftime("%Y-%m-%dT%H:%M:%SZ"),
        "corpus": {
            "path": str(LEDGER),
            "hash": corpus_hash,
            "events": len(events),
            "proposed_ids": len(proposed),
            "verified_ids": len(verified),
            "revoked_ids": len(revoked),
        },
        "specialization": specialization,
        "trajectory": trajectory,
        "coverage": coverage,
        "transfer": transfer,
    }

    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(manifest, indent=2))

    map_hash = hashlib.sha256(OUT.read_bytes()).hexdigest()
    print(json.dumps({
        "mastery_map": str(OUT),
        "hash": map_hash,
        "events": len(events),
        "specialization_rows": len(specialization),
        "trajectory_rows": len(trajectory),
        "coverage_rows": len(coverage),
        "transfer_rows": len(transfer),
    }, indent=2))

    return 0

if __name__ == "__main__":
    sys.exit(main())
