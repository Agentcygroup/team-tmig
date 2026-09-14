---- MODULE murmuration ----
EXTENDS Naturals, FiniteSets, Sequences

CONSTANTS Fellows, Units

VARIABLES ledger

TypeOK ==
    /\ ledger \in [Units -> [state: {"Proposed","Approved","Executed"},
                             approvals: SUBSET Fellows]]

Init ==
    ledger = [u \in Units |-> [state |-> "Proposed", approvals |-> {}]]

Approve(u, f) ==
    /\ u \in Units
    /\ f \in Fellows
    /\ ledger[u].state \in {"Proposed", "Approved"}
    /\ f \notin ledger[u].approvals
    /\ ledger' = [ledger EXCEPT ![u] = [state |-> "Approved",
                                        approvals |-> ledger[u].approvals \cup {f}]]

Execute(u) ==
    /\ u \in Units
    /\ ledger[u].state = "Approved"
    /\ Cardinality(ledger[u].approvals) >= 2
    /\ ledger' = [ledger EXCEPT ![u].state = "Executed"]

SafetyNoExecutionWithoutTwoFellows ==
    \A u \in Units:
        ledger[u].state = "Executed"
        => Cardinality(ledger[u].approvals) >= 2

\* Allow termination when no further progress is possible. This is a
\* stuttering step, not a real action. It prevents TLC from reporting a
\* deadlock when the system has reached a quiescent state.
Terminating ==
    /\ \A u \in Units: ledger[u].state \in {"Executed"} \/ Cardinality(ledger[u].approvals) >= 2
    /\ UNCHANGED ledger

Next ==
    \/ \E u \in Units, f \in Fellows: Approve(u, f)
    \/ \E u \in Units: Execute(u)
    \/ Terminating

Spec == Init /\ [][Next]_ledger

THEOREM Safety == Spec => []SafetyNoExecutionWithoutTwoFellows
====
