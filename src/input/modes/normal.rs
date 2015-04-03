use application::Application;
use input::commands::{Command, application, workspace, cursor, buffer};

pub fn handle(input: char) -> Command {
    let operation: fn(&mut Application, &char) = match input {
        '\t' => workspace::next_buffer,
        'q'  => application::exit,
        'j'  => cursor::move_down,
        'k'  => cursor::move_up,
        'h'  => cursor::move_left,
        'l'  => cursor::move_right,
        'x'  => buffer::delete,
        'i'  => application::switch_to_insert_mode,
        's'  => buffer::save,
        'H'  => cursor::move_to_start_of_line,
        'L'  => cursor::move_to_end_of_line,
        'f'  => application::switch_to_jump_mode,
        _    => do_nothing,
    };

    Command{ data: ' ', operation: operation }
}

pub fn do_nothing(app: &mut Application, _: &char) {
}