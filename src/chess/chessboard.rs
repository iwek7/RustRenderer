use std::collections::HashMap;
use std::rc::Rc;

use crate::api::drawable::Drawable;
use crate::api::resource_manager::ResourceManager;
use crate::chess::allowed_move::{AccompanyingMove, ActionType};
use crate::chess::field::{Field, FieldLogic};
use crate::chess::infrastructure::{PieceType, Side};
use crate::chess::piece::{Piece, PieceFactory, PieceLogic};
use crate::create_rect_coords;
use crate::maths::quadrangle::Quadrangle;
use crate::maths::vertex::TexturedVertexData;
use crate::renderer::RenderUtil;

pub struct Chessboard {
    board: Quadrangle<TexturedVertexData>,
    pieces: Vec<Piece>,
    piece_factory: PieceFactory,
    field_size: u32,
    board_size: u32,
    position: (f32, f32, f32),
    prev_mouse_pos: glam::Vec3,
    fields: Vec<Vec<Field>>,
    dragged_piece: Option<usize>,
    global_game_state: GlobalGameState,
}

impl Chessboard {
    pub fn new(resource_manager: Rc<ResourceManager>) -> Chessboard {
        let field_size = 1.0;
        let board_size = field_size * 8.0;
        let position = (0.0, 0.0, 0.0);

        let chessboard_texture = resource_manager.fetch_texture("textures/chessboard.png");
        let chessboard_shader = resource_manager.fetch_shader_program("shaders/texture");
        let possible_move_shader = resource_manager.fetch_shader_program("shaders/triangle");

        let quad = Quadrangle::new(
            create_rect_coords(position.clone(), (board_size, board_size),
                               &chessboard_texture.topology.get_sprite_coords(0, 0).unwrap()),
            [0, 1, 3, 1, 2, 3],
            Rc::clone(&chessboard_shader),
            Some(Rc::clone(&chessboard_texture)),
        );

        let piece_factory = PieceFactory::new(Rc::clone(&chessboard_shader));

        let mut fields = Vec::new();
        for row_idx in 0..8 as u32 {
            let mut row = Vec::new();
            for col_idx in 0..8 as u32 {
                row.push(Field::new(
                    col_idx,
                    row_idx,
                    col_idx as f32 * field_size + position.0,
                    row_idx as f32 * field_size + position.1,
                    field_size,
                    Rc::clone(&possible_move_shader),
                ));
            }
            fields.push(row);
        }

        return Chessboard {
            board: quad,
            pieces: vec!(),
            piece_factory,
            field_size: field_size as u32,
            board_size: board_size as u32,
            position,
            prev_mouse_pos: glam::Vec3::new(0.0, 0.0, 0.0),
            fields,
            dragged_piece: None,
            global_game_state: GlobalGameState::new(),
        };
    }

    pub fn init_pieces(&mut self, resource_manager: Rc<ResourceManager>) {
        let piece_size = (self.field_size as f32, self.field_size as f32);
        let pieces_sheet = resource_manager.fetch_sprite_sheet("textures/pieces.png", 2, 6);

        self.pieces.push(self.piece_factory.init_piece(PieceType::ROOK, Side::WHITE, Rc::clone(&pieces_sheet), self.get_field_by_name("A1"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::KNIGHT, Side::WHITE, Rc::clone(&pieces_sheet), self.get_field_by_name("B1"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::BISHOP, Side::WHITE, Rc::clone(&pieces_sheet), self.get_field_by_name("C1"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::QUEEN, Side::WHITE, Rc::clone(&pieces_sheet), self.get_field_by_name("D1"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::KING, Side::WHITE, Rc::clone(&pieces_sheet), self.get_field_by_name("E1"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::BISHOP, Side::WHITE, Rc::clone(&pieces_sheet), self.get_field_by_name("F1"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::KNIGHT, Side::WHITE, Rc::clone(&pieces_sheet), self.get_field_by_name("G1"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::ROOK, Side::WHITE, Rc::clone(&pieces_sheet), self.get_field_by_name("H1"), piece_size));

        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::WHITE, Rc::clone(&pieces_sheet), self.get_field_by_name("A2"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::WHITE, Rc::clone(&pieces_sheet), self.get_field_by_name("B2"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::WHITE, Rc::clone(&pieces_sheet), self.get_field_by_name("C2"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::WHITE, Rc::clone(&pieces_sheet), self.get_field_by_name("D2"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::WHITE, Rc::clone(&pieces_sheet), self.get_field_by_name("E2"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::WHITE, Rc::clone(&pieces_sheet), self.get_field_by_name("F2"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::WHITE, Rc::clone(&pieces_sheet), self.get_field_by_name("G2"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::WHITE, Rc::clone(&pieces_sheet), self.get_field_by_name("H2"), piece_size));

        self.pieces.push(self.piece_factory.init_piece(PieceType::ROOK, Side::BLACK, Rc::clone(&pieces_sheet), self.get_field_by_name("A8"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::KNIGHT, Side::BLACK, Rc::clone(&pieces_sheet), self.get_field_by_name("B8"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::BISHOP, Side::BLACK, Rc::clone(&pieces_sheet), self.get_field_by_name("C8"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::QUEEN, Side::BLACK, Rc::clone(&pieces_sheet), self.get_field_by_name("D8"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::KING, Side::BLACK, Rc::clone(&pieces_sheet), self.get_field_by_name("E8"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::BISHOP, Side::BLACK, Rc::clone(&pieces_sheet), self.get_field_by_name("F8"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::KNIGHT, Side::BLACK, Rc::clone(&pieces_sheet), self.get_field_by_name("G8"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::ROOK, Side::BLACK, Rc::clone(&pieces_sheet), self.get_field_by_name("H8"), piece_size));

        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::BLACK, Rc::clone(&pieces_sheet), self.get_field_by_name("A7"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::BLACK, Rc::clone(&pieces_sheet), self.get_field_by_name("B7"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::BLACK, Rc::clone(&pieces_sheet), self.get_field_by_name("C7"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::BLACK, Rc::clone(&pieces_sheet), self.get_field_by_name("D7"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::BLACK, Rc::clone(&pieces_sheet), self.get_field_by_name("E7"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::BLACK, Rc::clone(&pieces_sheet), self.get_field_by_name("F7"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::BLACK, Rc::clone(&pieces_sheet), self.get_field_by_name("G7"), piece_size));
        self.pieces.push(self.piece_factory.init_piece(PieceType::PAWN, Side::BLACK, Rc::clone(&pieces_sheet), self.get_field_by_name("H7"), piece_size));
    }

    fn get_field_position(&self, field: &Field) -> (f32, f32, f32) {
        (
            field.logic.col as f32 * self.field_size as f32 + self.position.0,
            field.logic.row as f32 * self.field_size as f32 + self.position.1,
            0.0
        )
    }

    pub fn handle_start_piece_dragging_attempt(&mut self, world_mouse_position: &glam::Vec3) {
        if self.is_game_over() {
            return;
        }
        for (i, piece_obj) in self.pieces.iter_mut().enumerate() {
            if piece_obj.is_mouse_over(world_mouse_position) {
                if piece_obj.logic.get_side() == self.global_game_state.get_side_to_move() {
                    self.dragged_piece = Some(i);
                    piece_obj.handle_start_drag();
                }
            }
        }
        if self.dragged_piece != None {
            {
                let allowed_moves = &mut self.pieces[self.dragged_piece.unwrap()].logic.get_all_allowed_moves(&self.create_chessboard_state());

                allowed_moves.get_moves().iter().for_each(|allowed_move| {
                    self.get_field_by_logic_mut(allowed_move.get_target()).update_with_allowed_move(&allowed_move.get_action_type());
                });
            }
            let occupied_field_logic = &self.pieces[self.dragged_piece.unwrap()].logic.get_occupied_field().clone();
            self.get_field_by_logic_mut(occupied_field_logic).is_current_field = true;
        }
        self.prev_mouse_pos = world_mouse_position.clone()
    }

    pub fn handle_piece_drop_attempt(&mut self, world_mouse_coords: &glam::Vec3, resource_manager: Rc<ResourceManager>) {
        println!("Handling piece drop at {:?}", world_mouse_coords);
        if self.is_game_over() {
            return;
        }
        match self.get_field_by_point(&(world_mouse_coords.x, world_mouse_coords.y)) {
            None => {
                if self.dragged_piece != None {
                    self.pieces[self.dragged_piece.unwrap()].return_to_initial_pos();
                }
            }
            Some(field) => {
                // todo: i dont know how to do this without two clones, thanks rust, I'm safe :D
                if self.dragged_piece != None {
                    let field_data = field.logic.clone();
                    let pos = field.get_position_3d();
                    let chessboard_state = &self.create_chessboard_state();

                    let action = &self.pieces[self.dragged_piece.unwrap()].handle_drop(
                        field_data.clone(),
                        pos,
                        chessboard_state,
                    );

                    let new_logic = &self.pieces[self.dragged_piece.unwrap()].logic.clone();

                    match action {
                        None => {}
                        Some(allowed_action) => {
                            self.global_game_state = self.global_game_state.with_switched_side();
                            match allowed_action.get_action_type() {
                                ActionType::CAPTURE { captured_piece } => { self.handle_piece_capture(&captured_piece.clone()) }
                                ActionType::COMPOSITE_MOVE { accompanying_move } => { self.handle_accompanying_move(accompanying_move); }
                                ActionType::PROMOTION => { self.handle_promotion(&new_logic, resource_manager) }
                                ActionType::CAPTURE_PROMOTION { captured_piece } => {
                                    self.handle_piece_capture(&captured_piece.clone());
                                    self.handle_promotion(&new_logic, resource_manager);
                                }
                                _ => {}
                            }

                            match allowed_action.get_action_type() {
                                ActionType::EN_PASSABLE_MOVE { en_passant_target_field } => {
                                    self.global_game_state = self.global_game_state.with_en_passant(en_passant_target_field.clone(), new_logic.clone())
                                }
                                _ => { self.global_game_state = self.global_game_state.with_disabled_en_passant() }
                            }
                        }
                    }
                }
            }
        }
        if self.create_chessboard_state().is_check_mated(self.global_game_state.get_side_to_move()) {
            self.global_game_state = self.global_game_state.with_winner(self.global_game_state.get_side_to_move().get_other());
        }

        self.clear_allowed_fields();
        self.dragged_piece = None;
        self.prev_mouse_pos = world_mouse_coords.clone()
    }

    pub fn handle_piece_dragging_attempt(&mut self, world_mouse_coords: &glam::Vec3) {
        if self.is_game_over() {
            return;
        }
        let drag_offset = &(
            (world_mouse_coords.x - self.prev_mouse_pos.x) as f32,
            (world_mouse_coords.y - self.prev_mouse_pos.y) as f32
        );

        if self.dragged_piece != None {
            self.pieces[self.dragged_piece.unwrap()].handle_drag_pointer_move(drag_offset);
        }
        self.prev_mouse_pos = world_mouse_coords.clone()
    }

    pub fn get_winner(&self) -> &Option<Side> {
        self.global_game_state.get_winner()
    }

    fn is_game_over(&self) -> bool {
        self.global_game_state.get_winner().is_some()
    }

    fn handle_piece_capture(&mut self, capture: &PieceLogic) {
        println!("Captured piece {}", capture);
        self.remove_piece_by_logic(capture);
    }

    fn handle_accompanying_move(&mut self, accompanying_move: &AccompanyingMove) {
        let source_field = accompanying_move.get_piece().get_occupied_field();
        let target_field_pos = self.get_field_by_logic_mut(accompanying_move.get_target()).get_position_3d();
        match self.get_piece_by_field(source_field) {
            None => { panic!("Accompanying move failed") }
            Some(piece) => {
                piece.force_move(accompanying_move.get_target().clone(), target_field_pos);
            }
        }
    }

    fn handle_promotion(&mut self, promoted_piece: &PieceLogic, resource_manager: Rc<ResourceManager>) {
        self.remove_piece_by_logic(promoted_piece);
        // todo: support promotion to different figures
        let piece_size = (self.field_size as f32, self.field_size as f32);
        let new_piece = self.piece_factory.init_piece(
            PieceType::QUEEN,
            promoted_piece.get_side().clone(),
            resource_manager.fetch_sprite_sheet("textures/pieces.png", 2, 6),
            self.get_field_by_logic(promoted_piece.get_occupied_field()),
            piece_size);
        self.pieces.push(new_piece);
    }

    fn remove_piece_by_logic(&mut self, piece_logic: &PieceLogic) {
        self.pieces.remove(
            self.pieces.iter().position(|piece| &piece.logic == piece_logic
            ).expect(&format!("Piece to remove {} not found", piece_logic)));
    }

    fn get_piece_by_field(&mut self, field_logic: &FieldLogic) -> Option<&mut Piece> {
        self.pieces.iter_mut().find(|piece| piece.logic.get_occupied_field() == field_logic)
    }

    fn get_field_by_logic_mut(&mut self, field_logic: &FieldLogic) -> &mut Field {
        &mut self.fields[field_logic.row as usize][field_logic.col as usize]
    }

    fn get_field_by_logic(&self, field_logic: &FieldLogic) -> &Field {
        &self.fields[field_logic.row as usize][field_logic.col as usize]
    }

    fn get_field_by_name(&self, name: &str) -> &Field {
        // hella inefficient, we just know don't need to check everywhere
        for row in self.fields.iter() {
            for f in row {
                if f.logic.name == name {
                    return f;
                }
            }
        }
        panic!("Asking for non existent field {}", name)
    }


    // could also move it to field.contains() or something
    fn get_field_by_point(&self, point: &(f32, f32)) -> Option<&Field> {
        return match self.get_field_coords_by_point(point) {
            None => None,
            Some(coords) => {
                let field = &self.fields[coords.0][coords.1];
                Some(field)
            }
        };
    }

    // todo: should be get field by point...
    fn get_field_coords_by_point(&self, point: &(f32, f32)) -> Option<(usize, usize)> {
        if point.0 < self.position.0 ||
            point.0 > self.position.0 + self.board_size as f32 ||
            point.1 < self.position.1 ||
            point.1 > self.position.1 + self.board_size as f32 {
            return None;
        }
        // todo rounding
        return Some(
            (
                ((point.1 - self.position.1).floor() as u32 / self.field_size) as usize,
                ((point.0 - self.position.0).floor() as u32 / self.field_size) as usize,
            )
        );
    }

    fn clear_allowed_fields(&mut self) {
        self.fields.iter_mut().for_each(|row| row.iter_mut().for_each(|field| field.clear_possible_moves_overlay()))
    }

    fn create_chessboard_state(&self) -> ChessboardState {
        let piece_logics = self.pieces.iter()
            .map(|piece| piece.logic.make_duplicate())
            .collect();
        return ChessboardState::new(piece_logics, self.global_game_state.clone());
    }
}

impl Drawable for Chessboard {
    fn render(&self, render_util: &RenderUtil) {
        self.board.render(render_util);
        self.fields.iter().for_each(|row| row.iter().for_each(|field| field.render(render_util)));
        self.pieces.iter().for_each(|piece| { piece.render(render_util) });
    }
}

pub struct ChessboardState {
    occupied_fields: HashMap<FieldLogic, PieceLogic>,
    global_game_state: GlobalGameState,
}

impl ChessboardState {
    pub fn new(pieces: Vec<PieceLogic>, global_game_state: GlobalGameState) -> ChessboardState {
        let mut occupied_fields: HashMap<FieldLogic, PieceLogic> = HashMap::new();
        for piece in pieces {
            occupied_fields.insert(piece.get_occupied_field().clone(), piece.make_duplicate());
        }
        return ChessboardState {
            occupied_fields,
            global_game_state,
        };
    }

    pub fn is_field_occupied(&self, field_data: &FieldLogic) -> bool {
        self.occupied_fields.get(field_data).is_some()
    }

    pub fn is_field_empty(&self, field_data: &FieldLogic) -> bool {
        !self.is_field_occupied(field_data)
    }

    pub fn get_piece_at(&self, field_data: &FieldLogic) -> Option<&PieceLogic> {
        self.occupied_fields.get(field_data)
    }

    pub fn get_global_game_state(&self) -> &GlobalGameState {
        &self.global_game_state
    }

    pub fn get_all_attacked_fields(&self, allied_side: &Side) -> Vec<FieldLogic> {
        return self.occupied_fields.values()
            .filter(|piece| piece.get_side() != allied_side)
            .flat_map(|piece| piece.get_all_attacked_fields(self).clone())
            .map(|action| action.get_target().clone())
            .collect();
    }

    pub fn is_in_check(&self, side: &Side) -> bool {
        let attacked_fields = self.get_all_attacked_fields(&side);
        attacked_fields.iter()
            .map(|field| self.occupied_fields.get(field))
            .filter(|possible_piece| possible_piece.is_some())
            .map(|possible_piece| possible_piece.unwrap())
            .filter(|piece| piece.get_side() == side)
            .any(|piece| piece.get_type() == &PieceType::KING)
    }

    pub fn move_piece_to(&self, from: &FieldLogic, target: &FieldLogic) -> ChessboardState {
        let mut new_occupied_fields = self.occupied_fields.clone();
        if let Some(piece) = new_occupied_fields.remove(from) {
            new_occupied_fields.insert(target.clone(), piece.move_to(target));
            ChessboardState {
                occupied_fields: new_occupied_fields,
                global_game_state: self.global_game_state.clone(),
            }
        } else {
            panic!("Trying to move piece from field it is not in in the first place...")
        }
    }

    pub fn is_check_mated(&self, side: &Side) -> bool {
        if self.is_in_check(side) {
            // check if any possible move prevents check mate
            for piece in self.get_all_pieces_of_side(side).iter() {
                let allowed_moves = piece.get_all_allowed_moves(self).get_moves().clone();
                for allowed_move in allowed_moves.iter() {
                    if !self.move_piece_to(piece.get_occupied_field(), allowed_move.get_target()).is_in_check(side) {
                        return false;
                    }
                }
            }
            return true;
        }
        return false;
    }

    fn get_all_pieces_of_side(&self, side: &Side) -> Vec<PieceLogic> {
        self.occupied_fields.values()
            .filter(|piece| piece.get_side() == side)
            .cloned()
            .collect()
    }
}

#[derive(Clone)]
pub struct GlobalGameState {
    side_to_move: Side,
    allowed_en_passant: Option<AllowedEnPassant>,
    winner: Option<Side>,
}

impl GlobalGameState {
    fn new() -> GlobalGameState {
        GlobalGameState {
            side_to_move: Side::WHITE,
            allowed_en_passant: None,
            winner: None,
        }
    }

    fn with_switched_side(&self) -> GlobalGameState {
        GlobalGameState {
            side_to_move: self.side_to_move.get_other(),
            allowed_en_passant: self.allowed_en_passant.clone(),
            winner: self.winner.clone(),
        }
    }

    fn with_en_passant(&self, target_field: FieldLogic, piece_to_capture: PieceLogic) -> GlobalGameState {
        GlobalGameState {
            side_to_move: self.side_to_move.clone(),
            allowed_en_passant: Some(AllowedEnPassant { target_field, piece_to_capture }),
            winner: self.winner.clone(),
        }
    }

    fn with_disabled_en_passant(&self) -> GlobalGameState {
        GlobalGameState {
            side_to_move: self.side_to_move.clone(),
            allowed_en_passant: None,
            winner: self.winner.clone(),
        }
    }

    fn with_winner(&self, side: Side) -> GlobalGameState {
        GlobalGameState {
            side_to_move: self.side_to_move.clone(),
            allowed_en_passant: self.allowed_en_passant.clone(),
            winner: Some(side),
        }
    }

    fn get_side_to_move(&self) -> &Side {
        &self.side_to_move
    }

    pub fn get_allowed_en_passant(&self) -> &Option<AllowedEnPassant> { &self.allowed_en_passant }

    pub fn get_winner(&self) -> &Option<Side> { &self.winner }
}

#[derive(Clone)]
pub struct AllowedEnPassant {
    target_field: FieldLogic,
    piece_to_capture: PieceLogic,
}

impl AllowedEnPassant {
    pub fn get_target_field(&self) -> &FieldLogic {
        &self.target_field
    }

    pub fn get_piece_to_capture(&self) -> &PieceLogic {
        &self.piece_to_capture
    }
}

