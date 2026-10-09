# COSC450_FinalProject
small_graph.csv	A 6-node graph with competing routes, small enough to check by hand.

city_roads.csv	A 15-edge graph with decimal weights and readable names like Airport and Stadium.

disconnected.csv	Two separate components (A-B-C and X-Y-Z). Going from A to Z should report “unreachable.”

edge_cases_valid.csv	Valid but tricky rows: duplicate edges with different weights, a self-loop, zero weight, a tiny weight, a huge weight, and scientific notation.

invalid_input.csv	Bad rows: missing fields, an empty name, a non-numeric weight, a negative weight, an empty weight, and an extra column. Your loader should report each one and keep going without crashing.

empty.csv and header_only.csv	A graph with zero edges.

large_graph.csv	1,000 nodes and about 5,000 edges, for the timing feature.