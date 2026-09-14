# NANPA Area Code Assignment Retrieval

Authority: North American Numbering Plan Administrator
Host: nationalnanpa.com (also nationalnanpa.com and nanpa.com)
Endpoints:
  https://nationalnanpa.com/reports/reports_cocodes_assign.html
  https://nationalnanpa.com/reports/area_code_assignments.csv
  https://nationalnanpa.com/reports/npa_report.csv
  https://nationalnanpa.com/nanp1/npa_report.csv

Failure observed: `curl: (6) Could not resolve host: nationalnanpa.com`

Diagnosis commands:
  nslookup nationalnanpa.com
  dig nationalnanpa.com
  host nationalnanpa.com
  curl -v --max-time 10 https://nationalnanpa.com

Possible causes:
  1 DNS resolver on the local network has no entry
  2 The host is blocked at the firewall
  3 The host is temporarily down (rare)

Fallback if still unresolvable:
  Fetch via a public DNS resolver: dig @1.1.1.1 nationalnanpa.com
  Fetch through a proxy
  Use the ITU-T E.164 country code list as a partial substitute
