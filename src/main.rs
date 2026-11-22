use iced::{
    Border, Color, Element, Font, Rectangle, Renderer, Shadow, Task, Theme, Vector, mouse, window,
    font::Weight,
    widget::{button, canvas, column, container, stack, text},
};

fn main() -> iced::Result {
   iced::application(
        || State {
            playing: false,
        },
        update,
        view,
    )
    .window(window::Settings {
          size: iced::Size::new(400.0, 600.0),
        decorations: false,
        transparent: true,
        ..Default::default()
    })
    .title("Mon iPod")
    .run()
}

#[derive(Debug, Clone)]
enum Message {
    Play,
    Pause,
    Next,
    Previous,
    Menu,
}

struct State {
    playing: bool,
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Play => {
            state.playing = true;
            println!("PLAY");
        }

        Message::Pause => {
            state.playing = false;
            println!("PAUSE");
        }

        Message::Next => {
            println!("NEXT");
        }

        Message::Previous => {
            println!("PREVIOUS");
        }

        Message::Menu => {
            println!("MENU");
        }
    }

    Task::none()
}

// Tailles
const ROUE: f32 = 250.0;
const MARGE_LABEL: f32 = 16.0;

// Couleurs
const BLEU: Color = Color::from_rgb8(0, 148, 225);
const BLEU_FONCE: Color = Color::from_rgb8(0, 108, 170);
const GRIS_LABEL: Color = Color::from_rgb8(150, 150, 150);

const GRAS: Font = Font {
    weight: Weight::Bold,
    ..Font::DEFAULT
};

fn view(state: &State) -> Element<'_, Message> {
    // ── Écran ──
    let entete = text("Now Playing")
        .size(12)
        .color(Color::from_rgb8(150, 150, 150));

    let titre = text("One More Time")
        .size(24)
        .font(GRAS);

    let artiste = text("Daft Punk")
        .size(16)
        .color(Color::from_rgb8(190, 190, 190));

    let screen = column![
        entete,
        titre,
        artiste,
    ]
    .spacing(6);

    let black_square = container(screen)
        .width(iced::Fill)
        .height(210)
        .padding(16)
        .style(|_theme| container::Style {
            background: Some(Color::from_rgb8(30, 30, 30).into()),
            text_color: Some(Color::WHITE),
            border: Border {
                color: Color::from_rgb8(10, 10, 10),
                width: 3.0,
                radius: 12.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba8(0, 0, 0, 0.35),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 6.0,
            },
            ..Default::default()
        });

    // ── Molette + boutons autour ──
    let play_pause = if state.playing { Message::Pause } else { Message::Play };

    let roue = stack![
        canvas(Roue).width(ROUE).height(ROUE),
        // MENU en haut
        container(bouton_roue("MENU", Message::Menu))
            .center_x(ROUE)
            .align_top(ROUE)
            .padding([MARGE_LABEL, 0.0]),
        // Précédent à gauche
        container(bouton_roue("◀◀", Message::Previous))
            .align_left(ROUE)
            .center_y(ROUE)
            .padding([0.0, MARGE_LABEL]),
        // Suivant à droite
        container(bouton_roue("▶▶", Message::Next))
            .align_right(ROUE)
            .center_y(ROUE)
            .padding([0.0, MARGE_LABEL]),
        // Play / Pause en bas
        container(bouton_roue("▶ ❚❚", play_pause))
            .center_x(ROUE)
            .align_bottom(ROUE)
            .padding([MARGE_LABEL, 0.0]),
    ];

    let contenu = column![
        black_square,
        container(roue).center_x(iced::Fill),
    ]
    .spacing(28);

    // ── Boîtier ──
    let ipod = container(contenu)
        .width(340)
        .height(570)
        .padding(24)
        .style(|_theme| container::Style {
            background: Some(BLEU.into()),
            text_color: Some(Color::WHITE),
            border: Border {
                color: BLEU_FONCE,
                width: 3.0,
                radius: 36.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba8(0, 0, 0, 0.3),
                offset: Vector::new(0.0, 8.0),
                blur_radius: 20.0,
            },
            ..Default::default()
        });

    container(ipod)
        .center_x(iced::Fill)
        .center_y(iced::Fill)
        .into()
}

// Bouton sans fond posé sur la molette (MENU, ◀◀, ▶▶, ▶ ❚❚)
fn bouton_roue(label: &str, message: Message) -> Element<'_, Message> {
    button(text(label).size(14).font(GRAS))
        .on_press(message)
        .padding([4, 8])
        .style(|_theme, status| {
            let couleur = match status {
                button::Status::Hovered | button::Status::Pressed => Color::from_rgb8(90, 90, 90),
                _ => GRIS_LABEL,
            };
            button::Style {
                background: None,
                text_color: couleur,
                border: Border {
                    radius: 8.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .into()
}

// La molette de l'iPod : un grand cercle clair avec le bouton du centre
struct Roue;

impl<Message> canvas::Program<Message> for Roue {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        // Le cercle prend toute la place, moins de quoi dessiner sa bordure
        let rayon = bounds.width.min(bounds.height) / 2.0 - 3.0;

        let molette = canvas::Path::circle(frame.center(), rayon);
        frame.fill(&molette, Color::from_rgb8(235, 235, 235));
        frame.stroke(
            &molette,
            canvas::Stroke::default()
                .with_width(3.0)
                .with_color(Color::from_rgb8(200, 200, 200)),
        );

        let centre = canvas::Path::circle(frame.center(), rayon * 0.36);
        frame.fill(&centre, Color::from_rgb8(30, 30, 30));
        frame.stroke(
            &centre,
            canvas::Stroke::default()
                .with_width(3.0)
                .with_color(Color::from_rgb8(70, 70, 70)),
        );

        vec![frame.into_geometry()]
    }
}
