use crate::{map, vector_2d::Vector2D};
use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window, Sdl};

pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;

struct Hit {
    map_box_value: u8,
    side: u8,
    ray_length: f64,
}

impl Hit {
    pub fn new(map_box_value: u8, side: u8, ray_length: f64) -> Hit {
        Hit {
            map_box_value,
            side,
            ray_length,
        }
    }
}

pub struct Camera {
    canvas: Canvas<Window>,
    position: Vector2D<f64>,
    direction: Vector2D<f64>,
    view_plane: Vector2D<f64>,
}

impl Camera {
    pub fn new(sdl_context: &Sdl) -> Camera {
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("RAY_CASTING", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Camera {
            canvas,
            position: map::STARTING_POSITION,
            direction: Vector2D::new(-1.0, 0.0),
            view_plane: Vector2D::new(0.0, 0.66),
        }
    }

    pub fn move_forward(&mut self, step: f64) {
        self.position.x += self.direction.x * step;
        self.position.y += self.direction.y * step
    }

    pub fn move_backward(&mut self, step: f64) {
        self.position.x -= self.direction.x * step;
        self.position.y -= self.direction.y * step;
    }

    pub fn rotate(&mut self, rotation: f64) {
        let current_direction = self.direction;
        let curretnt_view_plane = self.view_plane;

        self.direction.x =
            current_direction.x * rotation.cos() - current_direction.y * rotation.sin();
        self.direction.y =
            current_direction.x * rotation.sin() + current_direction.y * rotation.cos();
        self.view_plane.x =
            curretnt_view_plane.x * rotation.cos() - curretnt_view_plane.y * rotation.sin();
        self.view_plane.y =
            curretnt_view_plane.x * rotation.sin() + curretnt_view_plane.y * rotation.cos();
    }

    pub fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        self.canvas.clear();
        self.render_view_plane();
        self.canvas.present();
    }

    fn render_view_plane(&mut self) {
        for x in 0..SCREEN_WIDTH {
            let camera_plane_x_coordinate = (2.0 * (x as f64 / SCREEN_WIDTH as f64)) - 1.0;
            let ray_direction = Vector2D::new(
                self.direction.x + self.view_plane.x * camera_plane_x_coordinate,
                self.direction.y + self.view_plane.y * camera_plane_x_coordinate,
            );

            let ray_delta_distance = self.calculate_ray_delta_distance(&ray_direction);
            let ray_side_distance =
                self.calculate_ray_side_distance(&ray_direction, &ray_delta_distance);

            let hit = self.calculate_hit(ray_direction, ray_side_distance, ray_delta_distance);

            let wall_height = (SCREEN_HEIGHT as f64 / hit.ray_length) as i32;
            let wall_begin = std::cmp::max((SCREEN_HEIGHT as i32 / 2) - (wall_height / 2), 0);
            let wall_end = std::cmp::min(
                (SCREEN_HEIGHT as i32 / 2) + (wall_height / 2),
                SCREEN_HEIGHT as i32 - 1,
            );

            self.draw_line(x as i32, wall_begin, wall_end, self.pick_draw_color(&hit));
        }
    }

    fn calculate_ray_delta_distance(&self, ray_direction: &Vector2D<f64>) -> Vector2D<f64> {
        let x = if ray_direction.x == 0.0 {
            f64::INFINITY
        } else {
            (1.0 / ray_direction.x).abs()
        };

        let y = if ray_direction.y == 0.0 {
            f64::INFINITY
        } else {
            (1.0 / ray_direction.y).abs()
        };

        Vector2D::new(x, y)
    }

    fn calculate_ray_side_distance(
        &self,
        ray_direction: &Vector2D<f64>,
        ray_delta_distance: &Vector2D<f64>,
    ) -> Vector2D<f64> {
        let current_map_box = Vector2D::new(self.position.x as i32, self.position.y as i32);

        let x = ray_delta_distance.x
            * (if ray_direction.x < 0.0 {
                self.position.x - current_map_box.x as f64
            } else {
                (current_map_box.x + 1) as f64 - self.position.x
            });

        let y = ray_delta_distance.y
            * (if ray_direction.y < 0.0 {
                self.position.y - current_map_box.y as f64
            } else {
                (current_map_box.y + 1) as f64 - self.position.y
            });

        Vector2D::new(x, y)
    }

    fn calculate_hit(
        &self,
        ray_direction: Vector2D<f64>,
        mut ray_side_distance: Vector2D<f64>,
        ray_delta_distance: Vector2D<f64>,
    ) -> Hit {
        let mut current_map_box = Vector2D::new(self.position.x as i32, self.position.y as i32);

        let step = Vector2D::new(
            if ray_direction.x < 0.0 { -1 } else { 1 },
            if ray_direction.y < 0.0 { -1 } else { 1 },
        );

        let mut side: u8 = 0;
        let mut map_box_value: u8 = 0;

        while map_box_value == 0 {
            if ray_side_distance.x < ray_side_distance.y {
                ray_side_distance.x += ray_delta_distance.x;
                current_map_box.x += step.x;
                side = 0;
            } else {
                ray_side_distance.y += ray_delta_distance.y;
                current_map_box.y += step.y;
                side = 1;
            }

            if current_map_box.x >= 0
                && current_map_box.x < map::MAP_SIZE as i32
                && current_map_box.y >= 0
                && current_map_box.y < map::MAP_SIZE as i32
            {
                map_box_value =
                    map::MAPDATA[current_map_box.x as usize][current_map_box.y as usize];
            }
        }

        let ray_length = if side == 0 {
            ray_side_distance.x - ray_delta_distance.x
        } else {
            ray_side_distance.y - ray_delta_distance.y
        };

        Hit::new(map_box_value, side, ray_length)
    }

    fn pick_draw_color(&self, hit: &Hit) -> Color {
        let mut color = match hit.map_box_value {
            1 => Color::RGBA(0, 0, 255, 255),
            2 => Color::RGBA(0, 255, 0, 255),
            3 => Color::RGBA(255, 0, 0, 255),
            4 => Color::RGBA(255, 0, 255, 255),
            5 => Color::RGBA(125, 125, 0, 255),
            _ => panic!("The map contains an ID that has no assigned color!"),
        };

        if hit.side == 1 {
            color.r = ((color.r as u32 * 0xC0) >> 8) as u8;
            color.g = ((color.g as u32 * 0xC0) >> 8) as u8;
        }

        color
    }

    fn draw_line(&mut self, x_coordinate: i32, wall_begin: i32, wall_end: i32, color: Color) {
        self.canvas.set_draw_color(Color::RGB(32, 32, 32));
        self.canvas
            .draw_line(
                Point::new(x_coordinate, 0),
                Point::new(x_coordinate, wall_begin),
            )
            .expect("Should draw line");

        self.canvas.set_draw_color(color);
        self.canvas
            .draw_line(
                Point::new(x_coordinate, wall_begin),
                Point::new(x_coordinate, wall_end),
            )
            .expect("Should draw line");

        self.canvas.set_draw_color(Color::RGB(80, 80, 80));
        self.canvas
            .draw_line(
                Point::new(x_coordinate, wall_end),
                Point::new(x_coordinate, SCREEN_HEIGHT as i32 - 1),
            )
            .expect("Should draw line");
    }
}
