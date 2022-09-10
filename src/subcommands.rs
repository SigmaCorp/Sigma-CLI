use clap::Args;

#[derive(Args, Debug)]
#[clap(long_about = "Loguearse y almacenar la token para futuros usos")]
pub struct Login {
    // Usuario de Sigma
    pub usuario: String,
    // Pasword de Sigma
    pub password: String,
}

#[derive(Args, Debug)]
#[clap(long_about = "Busca datos de una persona a traves de su DNI")]
pub struct BuscarDNIStandard {
    // DNI perteneciente a la persona a buscar
    #[clap(value_parser)]
    pub dni: String,
}

#[derive(Args, Debug)]
#[clap(long_about = "Busca los celulares de una persona a traves de su DNI")]
pub struct BuscarCelualaresDNI {
    // DNI perteneciente a la persona a buscar
    #[clap(value_parser)]
    pub dni: String,
}

#[derive(Args, Debug)]
#[clap(long_about = "Busca el historial de vehiculos que ha tenido una patente")]
pub struct BuscarPatente {
    // Patente del vehiculo a buscar
    #[clap(value_parser)]
    pub patente: String,
}

#[derive(Args, Debug)]
#[clap(long_about = "Busca el historial de vehiculos de una persona a traves de su DNI")]
pub struct BuscarPatenteDNI {
    // DNI perteneciente a la persona a buscar
    #[clap(value_parser)]
    pub dni: String,
}

#[derive(Args, Debug)]
#[clap(long_about = "Busca emails y passwords leaked de una query")]
pub struct BuscarLeaks {
    // Dominio, email o keyword a buscar
    #[clap(value_parser)]
    pub query: String,
}

#[derive(Args, Debug)]
#[clap(long_about = "Busca datos exclusivos de una persona a traves de su DNI")]
pub struct BuscarDNIProfesional {
    // DNI perteneciente a la persona a buscar
    #[clap(value_parser)]
    pub dni: String,

    // Genero de la persona (1: Hombre, 2: Mujer, 3: Otro)
    #[clap(value_parser)]
    pub genero: usize,
}

#[derive(Args, Debug)]
#[clap(long_about = "Busca personas por su nombre y otros filtros")]
pub struct BuscarNombre {
    // Nombre de la persona a buscar
    #[clap(value_parser)]
    pub nombre: String,
    // Provincia de la persona (opcional)
    #[clap(short, long, value_parser)]
    pub provincia: Option<String>,
    // Localidad de la persona (opcional)
    #[clap(short, long, value_parser)]
    pub localidad: Option<String>,
    // Edad minima de la persona (opcional)
    #[clap(long, value_parser)]
    pub edadmin: Option<String>,
    // Edad maxima de la persona (opcional)
    #[clap(long, value_parser)]
    pub edadmax: Option<String>,
}

#[derive(Args, Debug)]
#[clap(long_about = "Busca el email de un celular Movistar")]
pub struct BuscarMovistar {
    // Numero de celular a buscar
    #[clap(value_parser)]
    pub numero: String,
}

#[derive(Args, Debug)]
#[clap(long_about = "Busca los datos de las personas que vivan en una direccion")]
pub struct BuscarVecinos {
    // Direccion a buscar
    #[clap(value_parser)]
    pub direccion: String,
}

#[derive(Args, Debug)]
#[clap(long_about = "Busca los titulares de un numero de celular")]
pub struct BuscarCelular {
    // Numero de celular a buscar
    #[clap(value_parser)]
    pub numero: String,
}

#[derive(Args, Debug)]
#[clap(long_about = "Busca el titular de un numero de celular si esta en vigencia")]
pub struct BuscarCelularesMagic {
    // Numero de celular a buscar
    #[clap(value_parser)]
    pub numero: String,
}

#[derive(Args, Debug)]
#[clap(long_about = "Busca el titular de una cuenta CBU a traves del numero CBU o alias")]
pub struct BuscarCBU {
    // CBU o alias a buscar
    #[clap(value_parser)]
    pub cbu: String,
}

#[derive(Args, Debug)]
#[clap(long_about = "Busca el titular de un email")]
pub struct BuscarEmail {
    // Email a buscar
    #[clap(value_parser)]
    pub email: String,
}
