/**
[test]

[[test.results]]
value = "NONE"

[[test.results]]
value = "NONE"

[[test.results]]
value = "[{ id: ⟨/thing⟩:one, owned_by: [⟨/thing⟩:two], owns: [] }]"

[[test.results]]
value = "[{ id: ⟨/thing⟩:two, owned_by: [], owns: [⟨/thing⟩:one] }]"

[[test.results]]
value = "[{ id: ⟨/thing⟩:one, owned_by: [⟨/thing⟩:two], owns: [] }, { id: ⟨/thing⟩:two, owned_by: [], owns: [⟨/thing⟩:one] }]"

[env.capabilities]
allow-experimental = ["record_references"]

*/

DEFINE FIELD owned_by ON TABLE `/thing` TYPE set<record<`/thing`>> reference DEFAULT [];
DEFINE FIELD owns ON TABLE `/thing` TYPE references<`/thing`, owned_by>;

CREATE `/thing`:one SET owned_by = [`/thing`:two];
CREATE `/thing`:two;

SELECT * FROM `/thing`;
