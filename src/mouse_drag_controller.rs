
pub struct MouseDragController<> {
    prev_mouse_pos: (f32, f32),
}

impl<> MouseDragController<> {
    pub fn new() -> MouseDragController<> {
        return MouseDragController {
            prev_mouse_pos: (0.0, 0.0)
        };
    }

    /**
    iterating over all those draggables is veeery inefficient
    but I can't hold reference to currently dragged object here
    as it violates only one mutable ref rule
     **/
    pub fn handle_event(&mut self, event: &sdl2::event::Event,
                        mouse_pos: &(f32, f32),
                        objects: &mut [&mut dyn Draggable]) {
        match event {
            sdl2::event::Event::MouseButtonDown { .. } => {
                for obj in objects.iter_mut() {
                    if obj.is_mouse_over(mouse_pos) {
                        obj.handle_start_drag()
                    }
                }
            }
            sdl2::event::Event::MouseButtonUp { .. } => {
                objects.iter_mut().for_each(|it| { it.handle_drop() })
            }
            sdl2::event::Event::MouseMotion { .. } => {
                objects.iter_mut()
                    .for_each(|it| {
                        it.handle_drag_pointer_move(&(
                            mouse_pos.0 - self.prev_mouse_pos.0,
                            mouse_pos.1 - self.prev_mouse_pos.1
                        ))
                    });
            }
            _ => {}
        }
        self.prev_mouse_pos = mouse_pos.clone()
    }
}

pub trait Draggable {
    fn is_mouse_over(&self, mouse_pos: &(f32, f32)) -> bool;
    fn handle_start_drag(&mut self);
    fn handle_drop(&mut self);
    fn handle_drag_pointer_move(&mut self, drag_offset: &(f32, f32));
}