use std::error::Error;
use std::i32;
use std::io::{self, Read, Write};
use std::result;
use std::str::{self, FromStr};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}

type Result<T> = result::Result<T, Box<Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut points: Vec<Point> = vec![];
    for line in input.lines() {
        let point = line
            .parse()
            .or_else(|err| err!("failed to parse '{:?}': {}", line, err))?;
        points.push(point);
    }

    part1(&points)?;
    Ok(())
}

fn part1(points: &[Point]) -> Result<()> {
    let mut consts = Constellations::shatter_all(points);
    while consts.step() {}
    writeln!(io::stdout(), "constellations: {}", consts.groups.len())?;
    Ok(())
}

#[derive(Clone, Debug)]
struct Constellations {
    groups: Vec<Constellation>,
}

impl Constellations {
    fn shatter_all(points: &[Point]) -> Constellations {
        let mut groups = vec![];
        for &p in points {
            groups.push(Constellation { points: vec![p] });
        }
        Constellations { groups }
    }

    fn step(&mut self) -> bool {
        for i in 0..self.groups.len() {
            for j in i + 1..self.groups.len() {
                if self.groups[i].is_connected(&self.groups[j]) {
                    self.merge(i, j);
                    return true;
                }
            }
        }
        false
    }

    fn merge(&mut self, i1: usize, i2: usize) {
        let g2 = self.groups.swap_remove(i2);
        self.groups[i1].join(&g2);
    }
}

#[derive(Clone, Debug)]
struct Constellation {
    points: Vec<Point>,
}

impl Constellation {
    fn join(&mut self, other: &Constellation) {
        self.points.extend(other.points.iter().cloned());
    }

    fn is_connected(&self, other: &Constellation) -> bool {
        for p in other.points.iter() {
            if self.is_point_connected(p) {
                return true;
            }
        }
        false
    }

    fn is_point_connected(&self, point: &Point) -> bool {
        for p in self.points.iter() {
            if point.distance(p) <= 3 {
                return true;
            }
        }
        false
    }
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    t: i32,
}

impl Point {
    fn distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs()
            + (self.y - other.y).abs()
            + (self.z - other.z).abs()
            + (self.t - other.t).abs()
    }
}

impl FromStr for Point {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Point> {
        let parts: Vec<&str> = s.trim().split(",").collect();
        if parts.len() != 4 {
            return err!("unrecognized point '{:?}'", s);
        }
        Ok(Point {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
            z: parts[2].parse()?,
            t: parts[3].parse()?,
        })
    }
}

fn lolzers() {
    println!(
        "
    
    Lorem ipsum dolor sit amet, consectetur adipiscing elit.
    Nunc elementum lacus at gravida condimentum.
    In sodales tellus eget elit congue tincidunt id sit amet purus.
    Donec malesuada quam et sagittis viverra.
    Nunc ac leo non lorem hendrerit maximus quis vitae nisi.

    Morbi luctus turpis eget sapien suscipit, non aliquet leo sollicitudin.
    Vivamus hendrerit eros ac lorem consequat, ac efficitur enim gravida.
    Sed maximus lectus a magna sodales vestibulum.
    In lobortis nibh nec neque iaculis egestas.
    Praesent cursus dolor in felis semper, vitae gravida ex consectetur.

    Praesent fermentum ex sit amet magna sagittis, eget venenatis urna lobortis.
    Donec iaculis metus at odio gravida, et imperdiet enim laoreet.
    In rhoncus ipsum blandit elit viverra, eu facilisis arcu condimentum.
    Praesent iaculis ex eu ligula posuere dignissim.
    Sed elementum dui id dictum ultricies.

    Sed vitae dui sed nisl dictum semper sit amet ut lacus.
    Nam congue est sed volutpat laoreet.
    Ut at lacus ac quam fermentum aliquam id vitae nisi.

    Aliquam quis leo nec sem scelerisque fringilla.
    Maecenas hendrerit ante ullamcorper vehicula ultricies.

    Sed ultricies odio ac velit ornare convallis.
    Ut consequat lorem a consectetur cursus.
    Quisque vitae urna nec dolor tristique interdum lacinia nec odio.

    Fusce consectetur lorem et dignissim cursus.
    Pellentesque posuere lorem sed felis tempus congue.
    Sed pulvinar elit ac diam scelerisque interdum.
    Ut tempus nisl sed sodales venenatis.
    Quisque auctor tellus quis sapien cursus lacinia.
    Integer ut tellus fringilla, molestie augue in, tristique lacus.

    Nunc in tortor sed ipsum consequat ultricies.
    Duis non leo id libero lobortis posuere.
    Fusce tempus arcu nec libero finibus porttitor.
    Maecenas tristique velit at nulla tempus aliquet.
    Praesent ultrices tellus vitae tellus convallis ultrices.

    In porta nisi quis eleifend sodales.
    Vivamus id eros at quam ornare lacinia.
    Cras bibendum purus sed suscipit tristique.
    Suspendisse non purus molestie, accumsan metus at, euismod massa.
    Mauris consequat quam semper porta molestie.

    Nulla non nibh vestibulum, bibendum odio quis, porttitor nibh.

    Ut quis mauris in eros tempus elementum.
    Nullam ultricies diam ornare, aliquet leo et, dignissim dui.
    Donec congue nibh at ante condimentum bibendum.
    Sed sed mauris eu elit posuere efficitur ac in sapien.

    Maecenas condimentum nisi id sem porta eleifend.
    Aliquam ullamcorper enim ut leo scelerisque, sit amet varius felis dictum.
    Maecenas a mauris accumsan, sodales tortor sed, sollicitudin est.
    Donec ut tellus non mi aliquet congue.
    Cras elementum enim eu ultrices tincidunt.
    Integer venenatis urna eget pretium ultrices.

    Suspendisse vel tortor a neque lacinia commodo eget quis mi.
    Nam luctus quam a ante sollicitudin egestas.
    Nulla suscipit ipsum ut dui maximus, eget eleifend metus condimentum.
    Maecenas at lectus a urna aliquam interdum sit amet quis ex.

    Vivamus eget arcu et quam efficitur tempus condimentum et odio.
    Praesent porta lacus vel dui volutpat, nec luctus nisi ultricies.
    Phasellus consequat ante sit amet euismod sagittis.
    Quisque ut arcu pulvinar, convallis velit ut, bibendum est.
    Nulla a nunc tempus metus lacinia mollis sit amet non eros.
    Donec tristique ex ac lorem viverra, nec egestas mi ullamcorper.

    Aliquam euismod urna ac mi mollis, vel convallis purus mollis.
    Proin viverra leo ut dolor pulvinar, at volutpat nisl volutpat.

    In lacinia magna non ornare faucibus.
    Phasellus ullamcorper orci eu libero finibus, eu aliquet lectus suscipit.

    Proin faucibus erat sed velit mollis, eget aliquet massa molestie.
    Suspendisse sed erat quis sapien dignissim gravida.
    Sed et tortor pulvinar, condimentum urna vel, euismod velit.
    Donec vel massa in leo volutpat semper.
    Phasellus nec diam pretium massa euismod mollis.

    Curabitur feugiat quam dapibus, accumsan magna vitae, tempus ante.
    Suspendisse sit amet eros maximus, egestas risus sed, egestas dolor.
    Maecenas cursus purus nec mi sagittis euismod.

    Sed varius massa ac volutpat ornare.
    Nam tempus arcu feugiat arcu rutrum, a rhoncus sem tincidunt.
    Suspendisse semper libero eget arcu cursus, in lobortis dolor elementum.

    Vestibulum quis elit egestas, placerat orci dapibus, porttitor dui.

    Donec rutrum velit id diam porttitor, sit amet dictum mauris euismod.

    In quis mauris consequat, aliquam est a, posuere mi.
    Duis sed nisi quis erat fermentum blandit quis a orci.

    Curabitur hendrerit lacus mollis eros molestie porta.
    Nunc dignissim enim eu mi gravida, vitae varius libero vestibulum.
    Duis ut nisi vitae urna porttitor semper.
    Praesent vitae est eget erat posuere bibendum.

    Donec imperdiet tellus in mauris dictum tristique.
    Cras eget felis laoreet, molestie lorem id, dignissim arcu.

    In vestibulum sapien et velit pellentesque, quis feugiat ligula aliquet.
    Suspendisse ut leo in tellus mattis aliquet.
    Donec blandit magna sit amet quam pharetra lacinia in vel sem.

    Vestibulum commodo dolor vitae hendrerit eleifend.

    Nunc non mauris blandit, blandit tellus vel, sagittis nisl.
    Mauris iaculis ipsum in quam egestas, nec ultricies nulla sagittis.

    Etiam elementum lectus sed auctor commodo.
    Cras ut risus mattis, convallis turpis eget, iaculis justo.
    Quisque posuere nisl eu sapien auctor, ac egestas sapien vehicula.
    Fusce tempor mi luctus, mattis dolor ut, placerat tortor.
    Nulla lacinia turpis ac mi venenatis, at dignissim dui aliquam.

    Vivamus hendrerit dolor et lacus euismod mattis.
    Donec venenatis massa ac elit vestibulum, at ullamcorper felis condimentum.
    Curabitur quis tellus nec risus faucibus molestie.
    Donec vel sapien ultricies, efficitur arcu id, scelerisque elit.
    Donec bibendum dui eget erat ullamcorper maximus.

    Aliquam nec est gravida mauris rutrum luctus nec eu neque.
    Vivamus non quam sed ante consectetur efficitur quis vitae nibh.
    Ut feugiat dolor volutpat, fermentum dui eu, blandit lectus.
    Vestibulum sed purus vitae sapien dictum dignissim.
    Nam in nisl dictum, pharetra tellus ac, laoreet urna.

    Etiam in justo vitae mi efficitur suscipit ac non quam.
    Phasellus euismod eros sed faucibus pellentesque.
    Praesent auctor nunc sit amet interdum auctor.

    Maecenas pretium turpis in erat consequat, at porttitor nulla laoreet.
    Duis id nibh nec felis luctus egestas at suscipit turpis.
    Nulla id lorem varius, elementum mauris nec, accumsan metus.
    Donec congue libero ut velit tincidunt, ut aliquet est dictum.
    Praesent hendrerit justo sit amet quam egestas dapibus.

    Donec non tortor nec sem ornare tristique.
    Aenean sit amet purus eu nibh molestie varius.
    Curabitur ullamcorper mauris vitae blandit porta.

    Etiam sed est consequat, convallis felis quis, consectetur velit.

    Praesent imperdiet massa ac erat commodo auctor.
    Nunc iaculis turpis et venenatis luctus.
    Phasellus non nunc sed eros vehicula elementum vitae a libero.
    Ut vel magna a enim aliquam posuere.

    Pellentesque pellentesque massa sit amet metus accumsan aliquam.
    Pellentesque vel dolor iaculis, faucibus dui malesuada, accumsan urna.

    Praesent lobortis odio ac erat porttitor varius.
    Cras blandit lacus id ultrices iaculis.

    Morbi sit amet magna imperdiet, varius tortor vel, pellentesque lectus.
    Proin porta urna at massa tempus, in accumsan turpis elementum.

    In vitae tellus at leo congue placerat.
    Donec ac ipsum nec nunc sodales blandit non id neque.
    Curabitur dignissim elit id lacus malesuada molestie.
    Proin rhoncus lectus sed dignissim venenatis.

    Aenean consequat massa at est volutpat molestie.
    Donec scelerisque sem eget imperdiet porta.

    Phasellus at sapien id ligula fringilla tincidunt.
    Suspendisse eget augue mollis odio tincidunt commodo eu in ipsum.

    In condimentum magna id arcu placerat, quis tristique ex finibus.
    Phasellus ac mauris at sapien hendrerit viverra quis vitae nunc.
    Donec at mauris eget magna sodales facilisis quis ut turpis.

    Quisque suscipit sem quis purus elementum malesuada.
    Phasellus pellentesque neque ut quam ullamcorper, a porta metus mollis.
    Vestibulum quis mauris suscipit, finibus justo nec, rhoncus odio.
    Nullam hendrerit neque in malesuada luctus.
    Nunc viverra quam imperdiet erat volutpat fringilla.

    Integer laoreet erat vel risus egestas, eu venenatis ipsum venenatis.
    Morbi molestie mauris vel neque imperdiet mattis.
    Aenean et erat id neque aliquet ultrices rutrum non purus.

    Duis scelerisque elit ut semper fermentum.
    Nam bibendum augue ultrices justo malesuada, a molestie dolor euismod.
    Cras bibendum urna ac sem pretium, vel ullamcorper massa sodales.

    Vivamus sed nulla sit amet orci consequat fringilla vitae id tellus.
    Mauris vel purus ac orci rutrum tincidunt volutpat eu velit.

    Proin efficitur augue at risus viverra, id tincidunt risus venenatis.
    Donec dignissim lorem sit amet odio fringilla, vitae fermentum odio accumsan.
    In euismod ex in quam accumsan, nec molestie tortor interdum.
    Cras sed erat a dolor ultrices dictum.
    Ut in enim et lorem facilisis semper a id sapien.

    Duis vehicula mi in augue congue, vel bibendum odio sagittis.
    Vestibulum a massa id nisl posuere scelerisque.
    Fusce eget lectus viverra, porttitor tortor vitae, ultrices ante.
    Vestibulum placerat lorem ac ante dictum, ut blandit erat sollicitudin.
    Fusce ut lectus sit amet est maximus scelerisque sit amet sit amet nisi.
    Vivamus sit amet leo ac arcu cursus tincidunt.

    Curabitur rutrum augue eu sem mollis, porttitor auctor sem ullamcorper.
    Sed et sem a neque suscipit iaculis.
    Nam viverra sapien a nunc vestibulum, nec egestas leo tempus.
    Etiam eget massa vitae odio imperdiet pharetra ac vitae mi.
    Sed ullamcorper lorem eget diam commodo, sed dapibus libero vulputate.

    Phasellus nec diam placerat, luctus tortor eu, lacinia lectus.
    Duis faucibus augue eu risus bibendum, ut pellentesque ante tempus.
    Vivamus in ligula tincidunt sapien tempor lacinia vitae non justo.
    Etiam tincidunt tortor quis sem ultricies, non pellentesque velit euismod.

    Donec vitae dui non ante lobortis rhoncus.
    Duis facilisis dolor at est convallis facilisis.
    Donec eget justo at lacus pulvinar ullamcorper.

    Etiam porta ligula vel magna vulputate efficitur.
    Cras eget turpis a magna egestas efficitur ac eu metus.
    Sed nec magna in sapien sollicitudin facilisis.
    Integer vel ligula gravida, facilisis nisi id, interdum lacus.
    Phasellus venenatis leo nec nisl tempor, eu congue arcu posuere.

    Sed quis risus eu libero efficitur rhoncus ac vel dolor.
    Praesent et lacus varius, porttitor felis non, luctus tellus.
    Ut nec ex congue, sollicitudin mauris sed, sollicitudin mauris.
    Vivamus sit amet libero at nunc lacinia accumsan.
    Donec facilisis leo vel velit pharetra, vitae ornare sem efficitur.

    Sed sit amet justo vitae odio accumsan venenatis facilisis sit amet augue.
    Cras imperdiet nisl at commodo luctus.
    Vestibulum mollis mi ac neque semper, ut condimentum ligula ornare.

    Suspendisse at dolor sit amet nisl maximus efficitur a sit amet leo.
    Donec ac massa sollicitudin turpis ornare scelerisque eget et leo.

    Curabitur rutrum nunc non elementum consectetur.
    Proin convallis ex sit amet lacus suscipit egestas.
    In vestibulum dolor quis ipsum maximus tempor.

    Duis molestie dolor in justo fringilla efficitur.
    Mauris consequat lacus et urna egestas, at sodales augue elementum.
    Integer ac quam molestie, dictum ex non, vehicula lacus.

    Ut eget tellus fermentum, pretium diam ut, molestie odio.
    Nam iaculis arcu in justo tincidunt, ac pulvinar quam lobortis.
    Nulla dignissim lectus non nunc congue, at auctor massa tincidunt.
    Ut at nibh mattis, imperdiet dui eget, aliquam odio.
    Phasellus eget enim id mi laoreet euismod.
    Aenean eget lectus at lorem aliquet vehicula laoreet a nulla.

    Vivamus congue felis vel facilisis condimentum.
    Nulla tincidunt libero nec justo semper scelerisque ut ac augue.

    Cras quis dui et turpis sodales tincidunt.
    Suspendisse hendrerit magna aliquam, semper quam id, scelerisque velit.

    Quisque ornare risus nec auctor condimentum.
    In sit amet mi sed arcu aliquam rutrum nec id nisl.
    Praesent eu lectus ultricies lectus efficitur tincidunt sit amet a quam.
    Mauris fringilla nibh ac ligula ultrices finibus.

    Vestibulum nec mauris aliquam, commodo quam vitae, finibus nibh.
    Proin malesuada nisl in purus sollicitudin consectetur at sit amet velit.
    Sed nec elit vehicula, tincidunt nunc et, dignissim tortor.

    Donec tristique tellus non odio dapibus cursus.
    Aenean cursus orci vitae magna lobortis, in tempus urna accumsan.
    Nunc ultrices tortor ut mauris gravida, sed placerat ligula ultrices.

    Pellentesque rutrum ante id urna vulputate fringilla.
    Suspendisse a leo hendrerit, fermentum nisi ultrices, ornare nunc.
    Nulla non dolor dapibus, fringilla elit vel, accumsan neque.

    Integer sagittis mauris vitae ullamcorper mattis.
    Cras et mi gravida, iaculis nibh at, placerat est.

    In non turpis a dolor sollicitudin scelerisque vel sed dui.
    Nullam nec diam ultrices, suscipit risus quis, scelerisque purus.

    Quisque gravida erat vel lectus fermentum euismod.
    Etiam lacinia turpis at lacus congue imperdiet.

    Nam vel elit sit amet eros laoreet lacinia.
    Quisque vel ipsum mollis nulla dignissim varius eget ut dui.
    Curabitur sit amet quam auctor sem sollicitudin sollicitudin.
    Nulla vitae lacus sit amet felis aliquet pharetra.

    Etiam vel odio vitae velit vestibulum posuere et a lorem.
    Cras mollis enim et porttitor consequat.

    Praesent convallis erat non turpis elementum feugiat.
    Vestibulum scelerisque metus quis mattis lobortis.

    Vivamus pharetra odio sed accumsan malesuada.
    Suspendisse faucibus elit faucibus ultricies suscipit.
    Donec hendrerit erat quis facilisis molestie.
    Praesent lobortis nulla iaculis elit luctus laoreet.
    Phasellus ut lectus rhoncus, tempus sem ultricies, condimentum nulla.
    Duis sed erat non augue porttitor vulputate.

    Proin eu ligula ut lorem fringilla imperdiet vel nec dolor.
    Ut feugiat dui id auctor efficitur.

    Praesent vestibulum lectus nec lectus convallis, vitae pharetra tellus convallis.
    Etiam sed tortor ut nibh luctus bibendum vel et ligula.

    Fusce imperdiet libero sit amet dui varius pharetra sed eu urna.
    Nullam ac mi at sem dictum ornare.
    Sed vehicula dui eu ligula dapibus efficitur.

    Nulla porttitor sapien nec quam pharetra eleifend.
    Aliquam ultrices ante et ipsum vulputate vestibulum scelerisque eget lectus.
    Sed consectetur sem eget est porttitor vulputate.
    Morbi vitae nibh eu dui mollis viverra eu sit amet ex.

    Aliquam posuere turpis vel ex aliquet ultrices.
    Aenean nec urna fermentum, eleifend augue dictum, dignissim ipsum.
    Nullam mollis libero id molestie porttitor.

    Nam a arcu feugiat, pulvinar augue sed, ullamcorper elit.

    Duis eu lectus cursus, volutpat massa nec, molestie lacus.
    Fusce auctor turpis id metus tincidunt, non varius purus placerat.
    Phasellus tincidunt odio sodales, bibendum odio id, venenatis purus.
    Aenean fringilla diam eu mollis tincidunt.
    Curabitur ac ligula sed lectus mattis eleifend eget id mauris.

    Maecenas cursus magna ac ipsum tincidunt, mattis convallis mi luctus.
    Vestibulum varius velit ut auctor vestibulum.
    Vestibulum viverra eros id velit viverra, condimentum sollicitudin velit venenatis.
    Maecenas tincidunt est non posuere tincidunt.
    In at urna et lorem eleifend pellentesque.

    Nulla at metus vehicula, vestibulum justo ac, pellentesque dui.
    Maecenas a ipsum eu augue elementum sodales et non mauris.

    Integer et erat laoreet, sodales lorem vitae, vulputate neque.
    Duis ultricies turpis id ultricies imperdiet.
    Nullam eu nisi gravida, faucibus nisl non, imperdiet ex.
    Suspendisse cursus sapien ut velit feugiat consectetur.

    Vestibulum maximus augue non leo sollicitudin elementum.
    Aliquam blandit dui in suscipit hendrerit.
    Vestibulum ut nulla sit amet ipsum lobortis egestas ac et ante.
    Integer sed odio commodo, finibus leo aliquam, egestas est.

    Nullam eget nunc eu arcu efficitur volutpat.
    Maecenas pellentesque metus et gravida dapibus.
    Sed blandit tellus eu faucibus laoreet.
    Quisque eu urna ac libero vestibulum placerat sit amet molestie arcu.
    In ac ante at est iaculis placerat.

    Maecenas eget arcu consequat, imperdiet nisi non, ullamcorper neque.
    Donec bibendum sem in volutpat finibus.
    Pellentesque ac elit vel dui ultricies molestie.
    Praesent faucibus sapien vitae lacus imperdiet iaculis.
    Sed ultrices nunc in euismod ultricies.

    Integer egestas nisl non nisi bibendum laoreet.
    Donec pulvinar dui a sem finibus accumsan in tempus turpis.
    In lobortis lectus et sem pellentesque pulvinar.
    Duis gravida tellus eu maximus porta.
    Praesent pretium purus in iaculis suscipit.
    Vivamus sed dolor sed ipsum eleifend pharetra nec a nulla.

    Phasellus a justo feugiat nunc elementum luctus in nec lectus.
    Fusce pellentesque augue sit amet magna maximus iaculis.
    Etiam vel augue et tortor placerat consequat venenatis vehicula nunc.

    Nullam efficitur orci eu neque pretium, et suscipit dui hendrerit.
    Nulla commodo orci at libero feugiat, a ornare mi rutrum.
    Donec varius mauris et ultrices cursus.
    Morbi mollis turpis eu tortor gravida, a fringilla mi porta.
    Phasellus lacinia urna sit amet elit vestibulum faucibus.

    Proin fermentum augue sed libero fermentum pharetra.
    Duis vel justo sit amet eros feugiat elementum.

    Maecenas sed ligula id mauris imperdiet condimentum.
    Suspendisse a erat in libero cursus pellentesque et vitae justo.
    Donec a erat ultrices, pulvinar tellus ut, ornare ligula.

    Phasellus elementum velit sit amet tellus fringilla hendrerit.
    Nulla id lorem mollis, rutrum velit vitae, tempus nisi.

    Maecenas ornare libero et metus interdum porttitor.

    Cras fermentum massa vel massa tempus, non vehicula mauris efficitur.
    Phasellus in risus non tellus porttitor placerat.
    Nullam commodo nunc vitae egestas convallis.

    Etiam at neque in dui finibus bibendum maximus quis turpis.
    Aenean ut felis ultrices, laoreet nunc mollis, maximus nisl.

    Vivamus fermentum eros dapibus viverra porta.
    Cras eu ipsum et lacus commodo efficitur.
    Vivamus sed nibh ullamcorper, condimentum dolor sed, vestibulum arcu.
    Integer vehicula risus et est sodales scelerisque.
    Donec dictum dui sit amet erat pulvinar, iaculis faucibus nisl placerat.

    Nullam a justo vitae metus laoreet placerat.
    Proin gravida nisl id lacus iaculis, laoreet molestie eros porttitor.
    Curabitur aliquam diam sit amet ipsum accumsan, quis sagittis lorem aliquet.
    Phasellus nec mauris non velit tristique ornare.

    Pellentesque dictum tortor in massa pellentesque imperdiet.
    Nunc sit amet nisl ultrices, commodo eros eget, sodales augue.

    Cras eget magna et enim rutrum facilisis.
    Duis ac justo sit amet turpis feugiat blandit vitae ut eros.
    Vestibulum sed nunc ac orci ornare tempus ac eget magna.
    Proin sed mi mattis, feugiat ligula in, tempus metus.

    Nulla aliquam sem at tristique tristique.
    Sed vestibulum dolor sit amet velit pharetra ultricies.

    Nam pharetra mauris non consectetur pretium.
    Curabitur placerat turpis a lacinia viverra.
    Curabitur blandit justo nec erat cursus, sit amet pulvinar arcu malesuada.
    Sed egestas urna vitae nulla consequat laoreet.

    Nunc gravida nisi eu facilisis aliquam.
    Pellentesque sed leo et dui lacinia auctor.
    Cras vitae purus ornare mi venenatis ultrices quis sed urna.
    Nullam vehicula ligula in metus tincidunt, eu molestie odio congue.
    Proin vel erat vestibulum, faucibus sem vitae, aliquet ex.

    Proin ornare leo et ipsum pulvinar porttitor.
    In euismod sem id porttitor vulputate.
    Etiam sit amet velit auctor, pulvinar orci eget, malesuada tortor.
    Sed vitae metus non nisi gravida ullamcorper.
    Suspendisse aliquet ante vel tortor tristique consectetur.

    Vestibulum quis elit non lectus ultrices maximus.
    Aenean imperdiet sem id velit rutrum ultrices.
    Donec posuere mi id nisi suscipit hendrerit.
    Vestibulum iaculis ligula sed aliquet facilisis.

    Sed convallis nunc in magna cursus, non faucibus velit lobortis.
    Fusce viverra urna vel porta lobortis.
    Nullam tristique purus a mi porttitor, at iaculis dui maximus.
    Maecenas dignissim dolor interdum, euismod nisl at, pellentesque est.
    Curabitur consectetur neque in dolor elementum pulvinar.
    Proin imperdiet felis ut tellus pellentesque, nec ornare mi fermentum.

    Vestibulum lacinia nulla viverra eleifend pellentesque.
    Integer ullamcorper libero sed mauris condimentum, a maximus arcu mattis.
    Morbi cursus velit et tellus eleifend, sed tincidunt velit suscipit.
    Duis feugiat sapien a vulputate bibendum.

    Fusce fringilla lectus ac molestie volutpat.
    Vivamus quis augue sed sapien imperdiet laoreet.
    Duis sed lorem pretium, consectetur lectus nec, lobortis nisi.
    Aliquam aliquet nulla sed diam fringilla aliquam.
    Ut ut quam aliquet, faucibus tortor quis, elementum est.

    Quisque tincidunt turpis id imperdiet accumsan.
    Aliquam luctus nunc at mauris rhoncus, eget eleifend tortor interdum.
    Sed sed felis in ligula aliquam vestibulum.
    Sed ut nunc at urna rutrum vehicula vel non neque.

    Praesent at dui nec eros tempor lobortis.
    Ut malesuada metus molestie, porttitor eros nec, volutpat justo.

    Aliquam feugiat tortor id nunc ornare placerat at in eros.

    Mauris bibendum erat non purus lacinia consequat.
    Proin consequat quam id egestas auctor.
    Donec nec mauris cursus, convallis libero sit amet, convallis ipsum.

    Donec sit amet ligula eu justo semper vestibulum at id elit.
    Nullam molestie neque in vulputate sollicitudin.

    Donec faucibus lectus ac luctus congue.
    Duis non metus ac sapien malesuada vehicula.
    Quisque fringilla risus at ex blandit, in tincidunt neque dictum.
    Aliquam eget odio at lectus congue malesuada.
    Pellentesque quis dolor luctus, tincidunt dui convallis, ultrices ante.

    Nulla a arcu at ex pharetra euismod sit amet quis nibh.
    Donec lacinia libero id bibendum elementum.
    Nunc ultrices diam sit amet interdum efficitur.
    Aliquam id diam cursus, sagittis nibh eu, vulputate nibh.
    Quisque hendrerit leo eu erat auctor pharetra.

    Maecenas efficitur sapien in ipsum finibus, ut rutrum felis finibus.
    In fringilla lectus sed placerat hendrerit.
    Integer at magna vitae diam ultrices consequat.
    Praesent eu urna at eros vulputate cursus vulputate in mauris.
    Fusce ornare magna et arcu volutpat, in vulputate eros mollis.

    Vestibulum posuere quam sagittis sollicitudin placerat.

    Donec imperdiet felis non aliquet ornare.
    Aenean pharetra tellus sed turpis placerat, non molestie justo tristique.
    Pellentesque et ligula in metus pharetra sodales eget vitae elit.
    Etiam aliquam magna ac vulputate lobortis.

    Vestibulum mollis augue in commodo varius.
    Phasellus sollicitudin sapien iaculis ullamcorper molestie.
    Morbi luctus ligula eget tortor facilisis, vitae suscipit nisi blandit.

    Cras a elit sagittis, tincidunt tortor sed, porttitor neque.
    Pellentesque feugiat ante in dapibus consequat.
    Nulla pretium nunc vel nibh fringilla, a auctor eros malesuada.

    Nullam pharetra tellus porta tortor efficitur ullamcorper.
    Aliquam laoreet massa at fermentum tincidunt.

    Nam viverra mauris in risus vehicula, a ornare elit bibendum.
    Nunc vitae leo faucibus, sollicitudin nibh non, efficitur eros.
    Suspendisse tempus neque eu lectus malesuada vestibulum.
    Donec eu eros quis libero scelerisque semper.
    Nulla quis ligula nec sapien euismod malesuada.

    Donec porta metus in consequat tincidunt.
    Maecenas faucibus libero venenatis arcu condimentum posuere.

    Donec interdum orci in sapien sollicitudin, vitae sodales sapien tempor.
    Aliquam nec dui quis velit lacinia accumsan quis ut ex.
    Nam maximus enim mattis justo lobortis vehicula.

    Suspendisse id metus molestie, lobortis enim sed, aliquam metus.
    Integer eget metus faucibus, efficitur felis non, vehicula velit.
    Nam imperdiet ante at tellus porttitor fringilla.
    Integer placerat mauris eu faucibus tincidunt.

    Sed dapibus quam quis arcu gravida consectetur.
    Fusce semper velit tincidunt pellentesque mollis.
    Quisque porttitor arcu vel nisl faucibus mollis.

    Quisque eu eros eget massa egestas ornare a nec metus.
    Mauris ultrices justo sed lectus dictum efficitur.
    Phasellus ut est et ante pellentesque gravida sed nec quam.
    Sed gravida mauris non porttitor facilisis.
    Aenean accumsan ligula sit amet sapien tincidunt malesuada.

    Sed rhoncus lorem vitae ipsum porta sollicitudin.
    Vivamus eget quam pulvinar erat laoreet semper.
    Praesent sit amet elit vulputate, vulputate justo ut, pulvinar enim.

    Phasellus volutpat turpis eget pulvinar suscipit.
    Sed aliquet sapien quis purus dignissim lacinia.
    Integer ut purus nec erat auctor ultrices.

    Ut cursus elit eu hendrerit commodo.
    Donec et massa nec arcu fermentum molestie.
    Vestibulum rhoncus neque convallis, volutpat lectus quis, maximus odio.

    Ut commodo est sit amet ex rhoncus, ut sollicitudin est gravida.
    Ut cursus lorem sit amet diam bibendum tincidunt.

    Duis eleifend odio eget nisi volutpat convallis non vitae metus.
    Integer sed dui id massa mattis auctor eget eu ipsum.
    Maecenas finibus mi eu mauris ultrices, at euismod arcu ultricies.
    Maecenas in eros sed lectus ultricies hendrerit in eu enim.
    Proin at nunc et tortor finibus pretium vitae sed nunc.
    Proin aliquet enim a mi tempus tincidunt.

    Morbi dapibus enim eget porta venenatis.
    Pellentesque vel nisi hendrerit, sodales neque sed, egestas mi.
    Ut elementum ante eget ante lacinia, ac ullamcorper turpis mollis.
    Suspendisse egestas turpis et risus efficitur fermentum.

    Proin aliquet eros sit amet lorem consectetur sollicitudin.
    Nulla maximus felis ut pharetra viverra.
    Donec eu turpis et orci pellentesque facilisis.
    Integer sollicitudin augue ut ex fringilla auctor.
    In euismod urna sit amet congue malesuada.
    Nullam ut est vitae orci dignissim dictum sit amet a est.

    Suspendisse pellentesque ligula in tincidunt sollicitudin.
    Integer vel justo tristique, tempus felis id, pulvinar tellus.
    Donec et elit lacinia, convallis quam et, tincidunt neque.

    Vestibulum tincidunt nunc vitae risus placerat euismod nec ac purus.
    Nam vitae sapien nec urna laoreet tincidunt eget ut sapien.

    Nunc ullamcorper massa ut nibh semper dignissim.
    Phasellus non urna non turpis cursus blandit.
    Pellentesque tincidunt lorem et lectus porta posuere.

    Duis et felis molestie, congue purus et, pellentesque ex.
    Nam ac orci in elit dapibus sagittis.
    Vivamus porta purus at tortor pulvinar, non rutrum neque vestibulum.

    In laoreet ipsum at magna molestie viverra.
    Nullam non risus in sem convallis consectetur et id erat.
    Sed condimentum dui eget quam tincidunt molestie malesuada eget purus.
    Nunc facilisis urna eu finibus dapibus.

    Donec ut lectus lobortis, pharetra quam sed, euismod libero.
    Quisque bibendum velit et blandit suscipit.

    Nulla dignissim neque porttitor tincidunt tincidunt.
    Nullam non tortor porttitor, suscipit velit ut, varius diam.
    Donec vel massa nec augue ultricies venenatis.
    Pellentesque vestibulum massa vel lectus tristique, sit amet rhoncus velit tempus.
    Nam vel dolor nec sem mollis convallis.

    Curabitur maximus ante eu augue ornare egestas.
    In fermentum lacus ut bibendum ornare.
    Nulla efficitur nibh id leo suscipit, ut vestibulum felis pulvinar.
    Donec non sem non ligula sollicitudin vulputate.

    Nulla scelerisque nulla at diam commodo, a rhoncus ligula consectetur.
    Donec eget ante lobortis, sodales enim ac, ultrices purus.
    Sed id velit non odio aliquet pulvinar sed in ex.
    Sed in dui sit amet sapien lacinia posuere.
    Sed ut orci a ante rutrum elementum.

    In in risus commodo, faucibus est sit amet, semper nisi.

    Integer eu orci et ante dictum scelerisque ac at risus.
    Duis efficitur risus vitae diam vestibulum, nec tempor ipsum vehicula.
    Sed a elit et dui dapibus eleifend id id velit.
    Sed pulvinar ante in erat consequat interdum.
    Vivamus ut nisi scelerisque, tincidunt lorem at, luctus erat.

    Fusce efficitur orci non euismod suscipit.
    Nunc a felis non tellus gravida vehicula.
    Fusce in mauris pulvinar, dictum nisi sit amet, tempus metus.
    Vestibulum porttitor sem sed felis feugiat suscipit.
    Vestibulum vel leo ornare velit pretium facilisis non facilisis tortor.
    Vivamus condimentum nisl quis augue pharetra efficitur.

    Nullam nec lacus in magna mattis venenatis lacinia nec dui.
    Donec mollis lacus et lacus sagittis pellentesque.
    Nunc quis risus consequat, fringilla lectus et, accumsan ipsum.

    Sed in nunc vitae lacus imperdiet tempus a sed ligula.
    Aliquam in arcu eleifend, sodales risus non, sollicitudin magna.

    Praesent a lacus elementum, commodo dui in, suscipit mauris.
    Vivamus pretium mi a arcu rutrum ultricies.

    Sed id turpis viverra, vehicula nisi quis, commodo orci.
    Donec sodales mi a lorem rhoncus, pharetra gravida risus dictum.
    In dapibus ipsum quis commodo tristique.
    Vestibulum egestas mi non mollis mattis.

    Maecenas sodales mi sollicitudin sem cursus, ut pellentesque ex porta.
    In lobortis augue id ligula pharetra viverra.

    Pellentesque id metus consequat, congue metus a, facilisis tellus.
    Duis convallis enim sodales, dignissim risus vel, ornare eros.
    Curabitur euismod lectus condimentum, condimentum eros a, faucibus lorem.

    Sed vel nisi vel sem fermentum maximus.

    "
    )
}
