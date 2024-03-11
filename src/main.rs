use std::{io,io::{Write}, net::{SocketAddr, TcpStream, ToSocketAddrs}};


struct IrcBuilder {
    password : Option<String>,
    nickmake : Option<String>,
    capabilites : Vec<String>
}

impl IrcBuilder {
    fn with_password(mut self , password : impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }
    fn with_nickname(mut self , nickname : impl Into<String>) -> Self {
        self.nickmake = Some(nickname.into());
        self
    }
    fn with_capabilites(mut self , capabilites : impl Into<String>) -> Self {
        self.capabilites.push(capabilites.into());
        self
    }

    fn connect(self, addr : impl ToSocketAddrs)-> Result<Irc,io::Error> {
        let con = TcpStream::connect(addr)?;
        let mut irc = Irc{con};
        if self.password.is_some() && self.nickmake.is_some() {
            irc.authenticate(self.password, self.nickmake)?;
        }
        Ok(irc)
    }
}

struct Irc {
    con : TcpStream
}

impl Irc {
    fn authenticate (&mut self,password : Option<String> , nickname : Option<String>) -> io::Result<()> {
        if let Some(password) = password {
            writeln!(self.con,"PASS {}",password)?;
        }
        if let Some(nickname) = nickname {
            writeln!(self.con,"NICK {}",nickname)?;
        }
        Ok(())
    }

    fn recive(&mut self){
        
    }
}

#[derive(Debug,Clone)]
pub struct PrivMsg {
    username : String,
    message : String
}

#[derive(Debug,Clone)]
pub enum Message{
    PrivMsg(PrivMsg)
}

fn main(){
    let _c = String::from("");
    println!("{_c}");
}