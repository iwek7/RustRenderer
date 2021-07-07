use std::marker::PhantomData;

use crate::maths::shapes_common::Side::{LEFT, NONE, RIGHT};
use crate::maths::vertex::VertexShaderDataSetter;
use crate::render_gl;
use crate::render_gl::buffer::{ArrayBuffer, ElementArrayBuffer, VertexArray};
use crate::render_gl::buffer;
use crate::texture::Texture;

pub trait Area {
    fn contains_point(&self, point: &(f32, f32)) -> bool;
    fn area(&self) -> f32;
    fn num_vertices(&self) -> usize;
}

pub fn is_point_within_convex_polygon(point: &(f32, f32), vertices: &Vec<(f32, f32)>) -> bool {
    let mut previous_side: Side = Side::NONE;
    let num_vertices = vertices.len();

    for idx in 0..num_vertices {
        let v1: &(f32, f32) = &vertices[idx];
        let v2: &(f32, f32) = if idx == num_vertices - 1 {
            &vertices[0]
        } else {
            &vertices[idx + 1]
        };
        // todo: why are those called affine
        let affine_segment = subtract_vectors(v2, v1);
        let affine_point = subtract_vectors(point, v1);
        let current_side = Side::calculate_side(&affine_segment, &affine_point);
        if !previous_side.is_on_the_same_side_as(&current_side) {
            return false;
        }
        previous_side = current_side;
    }
    return true;

    // this really should be on some vector object
    fn subtract_vectors(v1: &(f32, f32), v2: &(f32, f32)) -> (f32, f32) {
        return (v1.0 - v2.0, v1.1 - v2.1);
    }
}

// todo make enum ON_EDGE for case 0
#[derive(PartialEq, Debug)] // for comparisons
enum Side {
    NONE,
    RIGHT,
    LEFT,
}

impl Side {
    fn calculate_side(affine_segment: &(f32, f32), affine_point: &(f32, f32)) -> Side {
        let x = Side::cosine_sign(affine_segment, affine_point);
        return match x {
            _ if x < 0.0 => LEFT,
            _ if x > 0.0 => RIGHT,
            _ => NONE
        };
    }

    fn is_on_the_same_side_as(&self, other: &Side) -> bool {
        return match ((self == other) || (self == &NONE)) && (other != &NONE) {
            true => { true }
            false => { false }
        };
    }


    // wtf is this
    fn cosine_sign(v1: &(f32, f32), v2: &(f32, f32)) -> f32 {
        return v1.0 * v2.1 - v1.1 * v2.0;
    }
}

