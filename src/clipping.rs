use glam::Vec3;

use crate::mesh::Vertex;
use crate::{mesh::Triangle, utils::scale_color};
use crate::utils::*;

fn distance_point_plane(p: &Vec3, plane: &Vec3, plane_n: &Vec3) -> f32 {
    return plane_n.x * p.x + plane_n.y * p.y + plane_n.z * p.z - Vec3::dot(*plane_n, *plane);
    //return Vec3::dot(*plane_n, *p - *plane);
}

fn vector_intersect_plane(plane: &Vec3, plane_n: &Vec3, line_start: &Vec3, line_end: &Vec3, t: &mut f32) -> Vec3
{
    let plane_d = -Vec3::dot(*plane_n, *plane);
    let ad = Vec3::dot(*line_start, *plane_n);
    let bd = Vec3::dot(*line_end, *plane_n);
    *t = (-plane_d - ad) / (bd - ad);
    let line_start_to_end = *line_end - *line_start;
    let line_to_intersect = line_start_to_end * *t;
    return *line_start + line_to_intersect;
}

// Thank you @Javidx9
pub fn clip_triangle(tri: &Triangle, plane: &Vec3, plane_n: &Vec3) -> Vec::<Triangle> {
    let mut result = Vec::<Triangle>::new();

    let mut inside_points = Vec::<Vertex>::new();
    let mut outside_points = Vec::<Vertex>::new();

    let d0 = distance_point_plane(&tri.v[0].pos, &plane, &plane_n) > 0.0;
    let d1 = distance_point_plane(&tri.v[1].pos, &plane, &plane_n) > 0.0;
    let d2 = distance_point_plane(&tri.v[2].pos, &plane, &plane_n) > 0.0;

    // Checking points
    if d0 {
        inside_points.push(tri.v[0]);
    } else {
        outside_points.push(tri.v[0]);
    }
    if d1 {
        inside_points.push(tri.v[1]);
    } else {
        outside_points.push(tri.v[1]);
    }
    if d2 {
        inside_points.push(tri.v[2]);
    } else {
        outside_points.push(tri.v[2]);
    }

    if inside_points.len() == 3 {
        result.push(tri.clone());
    } else if inside_points.len() == 1 && outside_points.len() == 2 { 
        let mut new = tri.clone();
        let mut t = 0.0;

        new.v[0].pos = inside_points[0].pos;
        new.v[0].color = inside_points[0].color;
        new.v[0].texture = inside_points[0].texture;

        new.v[1].pos = vector_intersect_plane(plane, plane_n, &inside_points[0].pos, &outside_points[0].pos, &mut t);
        new.v[1].color = add_colors(scale_color(inside_points[0].color, 1.0-t), scale_color(outside_points[0].color, t));
        new.v[1].texture = t * (outside_points[0].texture - inside_points[0].texture) + inside_points[0].texture;

        new.v[2].pos = vector_intersect_plane(plane, plane_n, &inside_points[0].pos, &outside_points[1].pos, &mut t);
        new.v[2].color = add_colors(scale_color(inside_points[0].color, 1.0-t), scale_color(outside_points[1].color, t));
        new.v[2].texture = t * (outside_points[1].texture - inside_points[0].texture) + inside_points[0].texture;

        result.push(new);
    } else if inside_points.len() == 2 && outside_points.len() == 1 {
        let mut new_0 = tri.clone();
        let mut new_1 = tri.clone();
        let mut t = 0.0;

        new_0.v[0].pos = inside_points[0].pos;
        new_0.v[0].color = inside_points[0].color;
        new_0.v[0].texture = inside_points[0].texture;

        new_0.v[1].pos = inside_points[1].pos;
        new_0.v[1].color = inside_points[1].color;
        new_0.v[1].texture = inside_points[1].texture;

        new_0.v[2].pos = vector_intersect_plane(plane, plane_n, &inside_points[0].pos, &outside_points[0].pos, &mut t);
        new_0.v[2].color = add_colors(scale_color(inside_points[0].color, 1.0-t), scale_color(outside_points[0].color, t));
        new_0.v[2].texture = t * (outside_points[0].texture - inside_points[0].texture) + inside_points[0].texture;

        // Second triangle
        new_1.v[0].pos = inside_points[1].pos;
        new_1.v[0].color = inside_points[1].color;
        new_1.v[0].texture = inside_points[1].texture;

        new_1.v[1].pos = new_0.v[2].pos;
        new_1.v[1].color = new_0.v[2].color;
        new_1.v[1].texture = new_0.v[2].texture;

        new_1.v[2].pos = vector_intersect_plane(plane, plane_n, &inside_points[1].pos, &outside_points[0].pos, &mut t);
        new_1.v[2].color = add_colors(scale_color(inside_points[1].color, 1.0-t), scale_color(outside_points[0].color, t));
        new_1.v[2].texture = t * (outside_points[0].texture - inside_points[1].texture) + inside_points[1].texture;

        result.push(new_0);
        result.push(new_1);
    }

    return result;
}
