use std::io::{self, stdout, Stdout};
use std::time::Instant;


use crossterm::terminal::{
    disable_raw_mode,enable_raw_mode,EnterAlternateScreen,LeaveAlternateScreen,

};

use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;

use ratatui::prelude::*;

use ratatui::widgets::canvas::{Canvas,Painter,Shape};

use ratatui::widgets::{
    Block,BorderType,Borders,Gauge,List,ListItem,Padding,Paragraph,Sparkline,

};

use symbols::Marker;

use crate::agent::Agent;
use crate::game::Game;
use crate::nn::Net;
use crate::sim::GenerationSummary;

use crate::*;


const COLOR_WALLS:Color=Color::Indexed(137);
const COLOR_BODY:Color=Color::Indexed(140);
const COLOR_HEAD:Color=Color::White;
const COLOR_DEAD:Color=Color::Indexed(205);
const COLOR_FOOD:Color::LightGreen;

pub struct Viz{
    frame_count:u32,
    data:VizData,
    term:Terminal<CrosstermBackend<Stdout>>,
}

struct TermViz;
struct GameRender<'a>{
    game:&'a Game,
}
struct NNColors{
    disabled_color:Color,
    inp_colors:Vec<Color>,
    hidden_1_colors:Vec<Color>,
    hidden_2_colors:Vec<Color>,
    out_ colors:Vec<Color>,
}

struct VizData{
    agent:Option<Agent>,
    stats:GenerationSummary,
    sim_start_ts:Instant,
    scores:Vec<u64>,
    gen_times:Vec<u64>,
    mutation_rate:f64,
    mutation_magnitude:f64,

}


impl Viz{
    pub fn new()->io::Result<self>{
        Ok(Self{
            frame_count:0,
            data:VizData::default(),
            term:TermViz::init_terminal()?,

        })
    }
        pub fn update_brain(&mut self,new_brain:Net){
            self.data.agent=Some(Agent::with_brain(new_brain));


        }
        pub fn update_summary(&mut self,stats:GenerationSummary,mr:f64,mg:f64){
            self.data.stats=stats;
            self.data.mutation_rate=mr;
            self.data.mutation_magnitude=mg;
            self.data.scores.push(stats.gen_max_score as u64);
            self.data.gen_times.push((stats.time_elapsed_secs*1000)as u64);
            if self.data.scores.len()>VIZ_GRAPHS_LEN{
                self.data.scores.remove(0);

            }
            if self.data.gen_times.len()>VIZ_GRAPHS_LEN{
                self.data.gen_times.remove(0);

            }

            let agent =self.data.agent.as_mut().unwrap();
            let is_alive=agent.update();
            if !is_alive{
                self.data.agent=Some(Agent::with_brain(agent.brain.clone()));

            }
        }
        pub fn draw(&mut self){
            let _=self.term.draw(|f| TermViz::draw(f,&self.data));

        }
        pub fn restore_Terminal()->io::Result<()>{
            disable_raw_mode()?;
            stdout().execute(LeaveAlternateScreen)?;
            Ok(())
        }


    }
    //rataUI Terminal Rendering

    impl TermViz{
        fn init_terminal()->io::Result<Terminal<CrosstermBackend<Stdout>>>{
            enable_raw_mode()?;
            stdout().execute(EnterAlternateScreen)?;
            Terminal::new(CrosstermBackend::new(stdout()))
        }
        fn draw(f:&mut Frame,viz:&VizData){
            //Gen 0 , Viz agent not availables yet;
            if viz.agent.is_none(){
                f.render_widget(
                    TermViz::widget_raw_text("Running Gen 0 .please wait .....".to_string()),
                    f.size(),

                );
                return;

            }
            if IS_LOW_DETAIL_MODE{
                f.render_widget(
                    TermViz::widget_raw_text(TermViz::get_simple_render_text(&viz)),
                    f.size(),
                );
                return;

            }
        }

    }


}
