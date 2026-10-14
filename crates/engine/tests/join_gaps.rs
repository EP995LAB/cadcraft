//! JOIN across gaps: collinear lines become one line and arcs of one circle one arc (#1174).

use std::f64::consts::{FRAC_PI_2, PI};

use cadcraft_engine::Session;
use cadcraft_engine::doc::{EntityKind, Handle};
use cadcraft_engine::geom::Vec2;
use serde_json::{Value, json};

fn h(r: &Value) -> String {
    r["handle"].as_str().or_else(|| r["handles"][0].as_str()).unwrap().to_string()
}

fn kind(s: &Session, h: &str) -> EntityKind {
    s.doc().unwrap().entity(Handle::parse_hex(h).unwrap()).unwrap().kind.clone()
}

fn exists(s: &Session, h: &str) -> bool {
    s.doc().unwrap().entity(Handle::parse_hex(h).unwrap()).is_some()
}

fn count(s: &Session) -> usize {
    s.doc().unwrap().model.iter().count()
}

fn line(s: &mut Session, a: [f64; 2], b: [f64; 2]) -> String {
    h(&s.execute("line", &json!({ "points": [a, b] })).unwrap())
}

/// An arc of the circle at (50,50), radius 30, from `start` to `end` degrees counter-clockwise.
fn arc(s: &mut Session, start: f64, end: f64) -> String {
    h(&s.execute("arc", &json!({ "center": [50, 50], "radius": 30, "start": start, "end": end })).unwrap())
}

fn line_ends(s: &Session, h: &str) -> (Vec2, Vec2) {
    match kind(s, h) {
        EntityKind::Line(l) => (l.a.xy(), l.b.xy()),
        k => panic!("not a line: {k:?}"),
    }
}

/// (center, radius, start, end) of an arc, angles in radians.
fn arc_of(s: &Session, h: &str) -> (Vec2, f64, f64, f64) {
    match kind(s, h) {
        EntityKind::Arc(a) => (a.center.xy(), a.radius, a.start, a.end),
        k => panic!("not an arc: {k:?}"),
    }
}

fn circle_of(s: &Session, h: &str) -> (Vec2, f64) {
    match kind(s, h) {
        EntityKind::Circle(c) => (c.center.xy(), c.radius),
        k => panic!("not a circle: {k:?}"),
    }
}

fn near_angle(a: f64, b: f64) -> bool {
    let d = (a - b).rem_euclid(std::f64::consts::TAU);
    d < 1e-9 || d > std::f64::consts::TAU - 1e-9
}

fn assert_arc(s: &Session, h: &str, start: f64, end: f64) {
    let (c, r, a, b) = arc_of(s, h);
    assert!(c.near(Vec2::new(50.0, 50.0), 1e-9), "{c:?}");
    assert!((r - 30.0).abs() < 1e-9, "{r}");
    assert!(near_angle(a, start) && near_angle(b, end), "arc from {a} to {b}, wanted {start} to {end}");
}

// ---------------- #1174: lines ----------------

#[test]
fn join_collinear_lines_across_a_gap_makes_one_line() {
    // LINE 0,0 50,0 and LINE 120,0 150,0: one LINE from 0,0 to 150,0.
    let mut s = Session::new();
    let a = line(&mut s, [0.0, 0.0], [50.0, 0.0]);
    let b = line(&mut s, [120.0, 0.0], [150.0, 0.0]);
    let r = s.execute("join", &json!({ "handles": [a, b] })).unwrap();
    assert_eq!(h(&r), a, "the source line keeps its handle");
    let (p, q) = line_ends(&s, &a);
    assert!(p.near(Vec2::new(0.0, 0.0), 1e-9), "{p:?}");
    assert!(q.near(Vec2::new(150.0, 0.0), 1e-9), "{q:?}");
    assert_eq!(count(&s), 1);
    assert!(!exists(&s, &b));
}

#[test]
fn join_collinear_lines_in_any_order_overlapping_and_sloped() {
    // Three lines on y = x, picked out of order, two of them overlapping, the source pointing
    // back: one line over the whole extent.
    let mut s = Session::new();
    let a = line(&mut s, [30.0, 30.0], [10.0, 10.0]);
    let b = line(&mut s, [-20.0, -20.0], [-5.0, -5.0]);
    let c = line(&mut s, [20.0, 20.0], [60.0, 60.0]);
    let r = s.execute("join", &json!({ "handles": [a, b, c] })).unwrap();
    assert_eq!(h(&r), a);
    let (p, q) = line_ends(&s, &a);
    let (lo, hi) = (Vec2::new(-20.0, -20.0), Vec2::new(60.0, 60.0));
    assert!((p.near(lo, 1e-9) && q.near(hi, 1e-9)) || (p.near(hi, 1e-9) && q.near(lo, 1e-9)), "{p:?} {q:?}");
    assert_eq!(count(&s), 1);
}

#[test]
fn join_touching_collinear_lines_make_one_line_not_a_polyline() {
    let mut s = Session::new();
    let a = line(&mut s, [0.0, 0.0], [10.0, 0.0]);
    let b = line(&mut s, [10.0, 0.0], [20.0, 0.0]);
    s.execute("join", &json!({ "handles": [a, b] })).unwrap();
    let (p, q) = line_ends(&s, &a);
    assert!(p.near(Vec2::new(0.0, 0.0), 1e-9) && q.near(Vec2::new(20.0, 0.0), 1e-9), "{p:?} {q:?}");
    assert_eq!(count(&s), 1);
}

#[test]
fn join_lines_off_the_source_line_are_refused_across_a_gap() {
    // Parallel but not collinear, with a gap: nothing joins and both stay.
    let mut s = Session::new();
    let a = line(&mut s, [0.0, 0.0], [50.0, 0.0]);
    let b = line(&mut s, [120.0, 1.0], [150.0, 1.0]);
    assert!(s.execute("join", &json!({ "handles": [a, b] })).is_err());
    assert_eq!(count(&s), 2);
    assert!(exists(&s, &a) && exists(&s, &b));
    // A line crossing the source's line without touching it: the same.
    let c = line(&mut s, [70.0, -10.0], [70.0, 10.0]);
    assert!(s.execute("join", &json!({ "handles": [a, c] })).is_err());
    assert_eq!(count(&s), 3);
}

#[test]
fn join_lines_meeting_at_a_corner_still_make_a_polyline() {
    // Lines that are not collinear keep chaining end to end into a polyline.
    let mut s = Session::new();
    let a = line(&mut s, [0.0, 0.0], [5.0, 0.0]);
    let b = line(&mut s, [5.0, 0.0], [5.0, 5.0]);
    let r = s.execute("join", &json!({ "handles": [a, b] })).unwrap();
    assert!(matches!(kind(&s, &h(&r)), EntityKind::LwPolyline(p) if p.vertices.len() == 3));
    assert_eq!(count(&s), 1);
}

// ---------------- #1174: arcs ----------------

#[test]
fn join_arcs_of_one_circle_across_a_gap_makes_one_arc() {
    // ARC C 50,50 80,50 50,80 (0° to 90°) and ARC C 50,50 20,50 50,20 (180° to 270°): one arc
    // counter-clockwise from the source's start, 0 to 4.71239 rad.
    let mut s = Session::new();
    let a = arc(&mut s, 0.0, 90.0);
    let b = arc(&mut s, 180.0, 270.0);
    let r = s.execute("join", &json!({ "handles": [a, b] })).unwrap();
    assert_eq!(h(&r), a, "the source arc keeps its handle");
    assert_arc(&s, &a, 0.0, 1.5 * PI);
    assert_eq!(count(&s), 1);
    assert!(!exists(&s, &b));
    // Picked the other way round the second arc is the source: from 180° counter-clockwise
    // round to 90°.
    let mut s = Session::new();
    let a = arc(&mut s, 0.0, 90.0);
    let b = arc(&mut s, 180.0, 270.0);
    let r = s.execute("join", &json!({ "handles": [b, a] })).unwrap();
    assert_eq!(h(&r), b);
    assert_arc(&s, &b, PI, FRAC_PI_2);
    assert_eq!(count(&s), 1);
}

#[test]
fn join_arcs_touching_overlapping_or_running_back_over_the_source_make_one_arc() {
    // Touching.
    let mut s = Session::new();
    let a = arc(&mut s, 0.0, 90.0);
    let b = arc(&mut s, 90.0, 180.0);
    s.execute("join", &json!({ "handles": [a, b] })).unwrap();
    assert_arc(&s, &a, 0.0, PI);
    assert_eq!(count(&s), 1);
    // Overlapping, three arcs out of order.
    let mut s = Session::new();
    let a = arc(&mut s, 0.0, 120.0);
    let b = arc(&mut s, 200.0, 260.0);
    let c = arc(&mut s, 90.0, 210.0);
    s.execute("join", &json!({ "handles": [a, b, c] })).unwrap();
    assert_arc(&s, &a, 0.0, 260.0_f64.to_radians());
    assert_eq!(count(&s), 1);
    // An arc running back across the source's start: the result starts where that arc starts
    // and holds every piece.
    let mut s = Session::new();
    let a = arc(&mut s, 0.0, 90.0);
    let b = arc(&mut s, 300.0, 30.0);
    s.execute("join", &json!({ "handles": [a, b] })).unwrap();
    assert_arc(&s, &a, 300.0_f64.to_radians(), FRAC_PI_2);
    assert_eq!(count(&s), 1);
}

#[test]
fn join_arcs_covering_the_whole_circle_make_a_circle() {
    let mut s = Session::new();
    let a = arc(&mut s, 0.0, 180.0);
    let b = arc(&mut s, 180.0, 0.0);
    let r = s.execute("join", &json!({ "handles": [a, b] })).unwrap();
    assert_eq!(h(&r), a);
    let (c, radius) = circle_of(&s, &a);
    assert!(c.near(Vec2::new(50.0, 50.0), 1e-9) && (radius - 30.0).abs() < 1e-9, "{c:?} {radius}");
    assert_eq!(count(&s), 1);
    // Three arcs round the circle, the last one running back over the first's start.
    let mut s = Session::new();
    let a = arc(&mut s, 0.0, 100.0);
    let b = arc(&mut s, 100.0, 250.0);
    let c = arc(&mut s, 250.0, 10.0);
    s.execute("join", &json!({ "handles": [a, b, c] })).unwrap();
    circle_of(&s, &a);
    assert_eq!(count(&s), 1);
}

#[test]
fn join_arcs_of_different_circles_are_refused_across_a_gap() {
    let mut s = Session::new();
    let a = arc(&mut s, 0.0, 90.0);
    let b = h(&s.execute("arc", &json!({ "center": [50, 50], "radius": 20, "start": 180, "end": 270 })).unwrap());
    let c = h(&s.execute("arc", &json!({ "center": [51, 50], "radius": 30, "start": 180, "end": 270 })).unwrap());
    assert!(s.execute("join", &json!({ "handles": [a, b] })).is_err());
    assert!(s.execute("join", &json!({ "handles": [a, c] })).is_err());
    assert_eq!(count(&s), 3);
    assert_arc(&s, &a, 0.0, FRAC_PI_2);
    // Arcs of different circles that touch end to end still chain into a polyline.
    let mut s = Session::new();
    let a = h(&s.execute("arc", &json!({ "center": [0, 0], "radius": 10, "start": 0, "end": 90 })).unwrap());
    let b = h(&s.execute("arc", &json!({ "center": [0, 20], "radius": 10, "start": 270, "end": 0 })).unwrap());
    let r = s.execute("join", &json!({ "handles": [a, b] })).unwrap();
    assert!(matches!(kind(&s, &h(&r)), EntityKind::LwPolyline(_)));
    assert_eq!(count(&s), 1);
}

#[test]
fn join_at_the_command_line_names_the_line_or_arc() {
    let mut s = Session::new();
    line(&mut s, [0.0, 0.0], [50.0, 0.0]);
    line(&mut s, [120.0, 0.0], [150.0, 0.0]);
    s.script("JOIN ALL\n\n").unwrap();
    assert!(s.running.is_none(), "{}", s.prompt_text());
    assert!(s.log.iter().any(|l| l.contains("2 objects joined into 1 line")), "{:?}", s.log);
    assert_eq!(count(&s), 1);
    let mut s = Session::new();
    arc(&mut s, 0.0, 90.0);
    arc(&mut s, 180.0, 270.0);
    s.script("JOIN ALL\n\n").unwrap();
    assert!(s.running.is_none(), "{}", s.prompt_text());
    assert!(s.log.iter().any(|l| l.contains("2 objects joined into 1 arc")), "{:?}", s.log);
    assert_eq!(count(&s), 1);
}
