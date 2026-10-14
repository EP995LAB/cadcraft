//! A hatch from an internal point keeps the arcs of its boundary (#1167): a circle is stored as
//! two vertices with bulge 1, a polyline's or an ARC's curved side as one bulged vertex, as a
//! hatch made by selecting the object or an island already is.

use cadcraft_engine::Session;
use cadcraft_engine::doc::{EntityKind, Handle, Hatch, HatchLoop};
use cadcraft_engine::geom::{PI, Vec2};
use serde_json::{Value, json};

fn hatch(s: &Session, h: &str) -> Hatch {
    match &s.doc().unwrap().entity(Handle::parse_hex(h).unwrap()).unwrap().kind {
        EntityKind::Hatch(x) => x.clone(),
        k => panic!("not a hatch: {k:?}"),
    }
}

fn last_hatch(s: &Session) -> Hatch {
    let hs: Vec<Hatch> =
        s.doc().unwrap().model.iter().filter_map(|e| if let EntityKind::Hatch(h) = &e.kind { Some(h.clone()) } else { None }).collect();
    hs.last().cloned().expect("no hatch")
}

fn handle(v: &Value) -> String {
    v["handle"].as_str().unwrap().to_string()
}

/// AREA's Object option on `h`.
fn area(s: &mut Session, h: &str) -> f64 {
    s.execute("area", &json!({ "handles": [h] })).unwrap()["area"].as_f64().unwrap()
}

/// The loop is a circle: two vertices, both with bulge 1, opposite each other.
fn assert_circle(l: &HatchLoop, center: Vec2, radius: f64) {
    assert_eq!(l.vertices.len(), 2, "{:?}", l.vertices);
    for v in &l.vertices {
        assert!((v.bulge - 1.0).abs() < 1e-9, "{:?}", l.vertices);
        assert!((v.p.dist(center) - radius).abs() < 1e-6, "{:?}", l.vertices);
    }
    assert!(l.vertices[0].p.mid(l.vertices[1].p).near(center, 1e-6), "{:?}", l.vertices);
}

/// The loop is a 100×50 rectangle whose right side is a semicircle: four vertices, the one at
/// (100, 0) with bulge 1 towards (100, 50), the others straight.
fn assert_rounded_rectangle(l: &HatchLoop) {
    let vs = &l.vertices;
    assert_eq!(vs.len(), 4, "{vs:?}");
    let i = vs.iter().position(|v| v.p.near(Vec2::new(100.0, 0.0), 1e-6)).unwrap_or_else(|| panic!("{vs:?}"));
    assert!((vs[i].bulge - 1.0).abs() < 1e-9, "{vs:?}");
    assert!(vs[(i + 1) % 4].p.near(Vec2::new(100.0, 50.0), 1e-6), "{vs:?}");
    assert!(vs.iter().enumerate().all(|(j, v)| j == i || v.bulge.abs() < 1e-9), "{vs:?}");
}

#[test]
fn internal_point_inside_a_circle_keeps_it_as_two_bulged_vertices() {
    let mut s = Session::new();
    let c = handle(&s.execute("circle", &json!({ "center": [50, 50], "radius": 40 })).unwrap());
    let h = handle(&s.execute("hatch", &json!({ "points": [[50, 50]], "pattern": "SOLID" })).unwrap());
    let hh = hatch(&s, &h);
    assert_eq!(hh.loops.len(), 1);
    assert_circle(&hh.loops[0], Vec2::new(50.0, 50.0), 40.0);
    assert!(hh.associative, "a loop that is where its circle is keeps the link");
    assert_eq!(hh.loops[0].sources, vec![Handle::parse_hex(&c).unwrap()]);
    assert!((area(&mut s, &h) - PI * 1600.0).abs() < 1e-6, "{}", area(&mut s, &h));
    // Typed at the command line, as in the report.
    let mut s = Session::new();
    s.script("CIRCLE 50,50 40\n-HATCH P SOLID\n50,50\n\n").unwrap();
    assert!(s.running.is_none(), "{}", s.prompt_text());
    assert_circle(&last_hatch(&s).loops[0], Vec2::new(50.0, 50.0), 40.0);
}

#[test]
fn internal_point_keeps_the_arc_of_a_polyline_boundary() {
    let mut s = Session::new();
    s.script("PLINE 0,0 100,0 A 100,50 L 0,50 C\n").unwrap();
    assert!(s.running.is_none(), "{}", s.prompt_text());
    let p = s.doc().unwrap().model.iter().next().unwrap().handle.hex();
    assert!(matches!(s.doc().unwrap().entity(Handle::parse_hex(&p).unwrap()).unwrap().kind, EntityKind::LwPolyline(_)));
    let h = handle(&s.execute("hatch", &json!({ "points": [[50, 25]], "pattern": "SOLID" })).unwrap());
    let hh = hatch(&s, &h);
    assert_eq!(hh.loops.len(), 1);
    assert_rounded_rectangle(&hh.loops[0]);
    let want = area(&mut s, &p);
    assert!((want - (5000.0 + PI * 625.0 / 2.0)).abs() < 1e-6, "{want}");
    assert!((area(&mut s, &h) - want).abs() < 1e-6, "{} vs {want}", area(&mut s, &h));
}

#[test]
fn internal_point_keeps_an_arc_among_lines_and_follows_them() {
    let mut s = Session::new();
    let mut hs = Vec::new();
    for pts in [[[0.0, 0.0], [100.0, 0.0]], [[100.0, 50.0], [0.0, 50.0]], [[0.0, 50.0], [0.0, 0.0]]] {
        hs.push(s.execute("line", &json!({ "points": pts })).unwrap()["handles"][0].as_str().unwrap().to_string());
    }
    hs.push(handle(&s.execute("arc", &json!({ "center": [100, 25], "radius": 25, "start": -90, "end": 90 })).unwrap()));
    let h = handle(&s.execute("hatch", &json!({ "points": [[50, 25]], "pattern": "SOLID" })).unwrap());
    let hh = hatch(&s, &h);
    assert_eq!(hh.loops.len(), 1);
    assert_rounded_rectangle(&hh.loops[0]);
    assert_eq!(hh.loops[0].sources.len(), 4);
    assert!((area(&mut s, &h) - (5000.0 + PI * 625.0 / 2.0)).abs() < 1e-6, "{}", area(&mut s, &h));
    // Associative: traced again after its objects move, still with the arc.
    s.execute("move", &json!({ "handles": hs, "delta": [0, 100] })).unwrap();
    let hh = hatch(&s, &h);
    assert!(hh.associative);
    let vs = &hh.loops[0].vertices;
    assert_eq!(vs.len(), 4, "{vs:?}");
    let i = vs.iter().position(|v| v.p.near(Vec2::new(100.0, 100.0), 1e-6)).unwrap_or_else(|| panic!("{vs:?}"));
    assert!((vs[i].bulge - 1.0).abs() < 1e-9, "{vs:?}");
    assert!((area(&mut s, &h) - (5000.0 + PI * 625.0 / 2.0)).abs() < 1e-6, "{}", area(&mut s, &h));
}

#[test]
fn islands_and_selected_objects_stay_exact() {
    let mut s = Session::new();
    s.execute("rectang", &json!({ "p1": [0, 0], "p2": [100, 100] })).unwrap();
    let c = handle(&s.execute("circle", &json!({ "center": [50, 50], "radius": 20 })).unwrap());
    // Two ARCs making a circle: an island of open curves.
    s.execute("arc", &json!({ "center": [20, 20], "radius": 5, "start": 0, "end": 180 })).unwrap();
    s.execute("arc", &json!({ "center": [20, 20], "radius": 5, "start": 180, "end": 360 })).unwrap();
    let h = handle(&s.execute("hatch", &json!({ "points": [[5, 50]], "pattern": "SOLID" })).unwrap());
    let hh = hatch(&s, &h);
    assert_eq!(hh.loops.len(), 3, "{:?}", hh.loops);
    assert_eq!(hh.loops[0].vertices.len(), 4);
    let island = hh.loops.iter().find(|l| !l.outer && l.sources == vec![Handle::parse_hex(&c).unwrap()]).unwrap_or_else(|| panic!("{:?}", hh.loops));
    assert_circle(island, Vec2::new(50.0, 50.0), 20.0);
    let arcs = hh.loops.iter().find(|l| !l.outer && l.sources.len() == 2).unwrap_or_else(|| panic!("{:?}", hh.loops));
    assert_circle(arcs, Vec2::new(20.0, 20.0), 5.0);
    assert!((area(&mut s, &h) - (10000.0 - PI * 400.0 - PI * 25.0)).abs() < 1e-6, "{}", area(&mut s, &h));
    // Selecting the circle.
    let h = handle(&s.execute("hatch", &json!({ "handles": [c], "pattern": "SOLID" })).unwrap());
    assert_circle(&hatch(&s, &h).loops[0], Vec2::new(50.0, 50.0), 20.0);
}

#[test]
fn a_circle_cut_by_a_line_keeps_its_arc_on_both_sides() {
    let mut s = Session::new();
    s.execute("circle", &json!({ "center": [0, 0], "radius": 10 })).unwrap();
    s.execute("line", &json!({ "points": [[5, -20], [5, 20]] })).unwrap();
    // Chord at distance 5: the small segment's area, and the rest of the disc.
    let small = 100.0 * (0.5f64).acos() - 5.0 * 75.0f64.sqrt();
    let big = handle(&s.execute("hatch", &json!({ "points": [[-5, 0]], "pattern": "SOLID" })).unwrap());
    let vs = &hatch(&s, &big).loops[0].vertices;
    assert_eq!(vs.len(), 3, "two half-arcs and the chord: {vs:?}");
    assert_eq!(vs.iter().filter(|v| v.bulge.abs() < 1e-9).count(), 1, "{vs:?}");
    assert!((area(&mut s, &big) - (PI * 100.0 - small)).abs() < 1e-6, "{}", area(&mut s, &big));
    let little = handle(&s.execute("hatch", &json!({ "points": [[7, 0]], "pattern": "SOLID" })).unwrap());
    let vs = &hatch(&s, &little).loops[0].vertices;
    assert_eq!(vs.len(), 2, "one arc and the chord: {vs:?}");
    assert!((area(&mut s, &little) - small).abs() < 1e-6, "{}", area(&mut s, &little));
}
