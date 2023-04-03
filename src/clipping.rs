use glam::Vec3;

use crate::mesh::Triangle;

fn distance_point_plane(p: &Vec3, plane: &Vec3, plane_n: &Vec3) -> f32 {
    return plane_n.x * p.x + plane_n.y * p.y + plane_n.z * p.z - Vec3::dot(*plane_n, *plane);
    //return Vec3::dot(*plane_n, *p - *plane);
}

fn vector_intersect_plane(plane: &Vec3, plane_n: &Vec3, line_start: &Vec3, line_end: &Vec3) -> Vec3
{
    let plane_d = -Vec3::dot(*plane_n, *plane);
    let ad = Vec3::dot(*line_start, *plane_n);
    let bd = Vec3::dot(*line_end, *plane_n);
    let t = (-plane_d - ad) / (bd - ad);
    let line_start_to_end = *line_end - *line_start;
    let line_to_intersect = line_start_to_end * t;
    return *line_start + line_to_intersect;
}

// Thank you @Javidx9
pub fn clip_triangle(tri: &Triangle, plane: &Vec3, plane_n: &Vec3) -> Vec::<Triangle> {
    let mut result = Vec::<Triangle>::new();

    let mut inside_points = Vec::<Vec3>::new();
    let mut outside_points = Vec::<Vec3>::new();

    let d0 = distance_point_plane(&tri.pos[0], &plane, &plane_n) > 0.0;
    let d1 = distance_point_plane(&tri.pos[1], &plane, &plane_n) > 0.0;
    let d2 = distance_point_plane(&tri.pos[2], &plane, &plane_n) > 0.0;

    // Checking points
    if d0 {
        inside_points.push(tri.pos[0]);
    } else {
        outside_points.push(tri.pos[0]);
    }
    if d1 {
        inside_points.push(tri.pos[1]);
    } else {
        outside_points.push(tri.pos[1]);
    }
    if d2 {
        inside_points.push(tri.pos[2]);
    } else {
        outside_points.push(tri.pos[2]);
    }

    // Whole triangle is inside
    if inside_points.len() == 3 {
        result.push(tri.clone());
    } else if inside_points.len() == 1 && outside_points.len() == 2 {
        // Triangle should be clipped. As two points lie outside
        // the plane, the triangle simply becomes a smaller triangle
        let mut new = tri.clone();

        // The inside point is valid, so keep that...
        new.pos[0] = inside_points[0];

        // but the two new points are at the locations where the 
        // original sides of the triangle (lines) intersect with the plane
        new.pos[1] = vector_intersect_plane(plane, plane_n, &inside_points[0], &outside_points[0]);
        new.pos[2] = vector_intersect_plane(plane, plane_n, &inside_points[0], &outside_points[1]);
        result.push(new);
    } else if inside_points.len() == 2 && outside_points.len() == 1 {
        // Triangle should be clipped. As two points lie inside the plane,
        // the clipped triangle becomes a "quad". Fortunately, we can
        // represent a quad with two new triangles
        let mut new_0 = tri.clone();
        let mut new_1 = tri.clone();

        // The first triangle consists of the two inside points and a new
        // point determined by the location where one side of the triangle
        // intersects with the plane
        new_0.pos[0] = inside_points[0];
        new_0.pos[1] = inside_points[1];
        new_0.pos[2] = vector_intersect_plane(plane, plane_n, &inside_points[0], &outside_points[0]);

        // The second triangle is composed of one of the inside points, a
        // new point determined by the intersection of the other side of the 
        // triangle and the plane, and the newly created point above
        new_1.pos[0] = inside_points[1];
        new_1.pos[1] = new_0.pos[2];
        new_1.pos[2] = vector_intersect_plane(plane, plane_n, &inside_points[1], &outside_points[0]);
        result.push(new_0);
        result.push(new_1);
    }

    return result;
}
