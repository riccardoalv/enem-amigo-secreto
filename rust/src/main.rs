use rand::Rng;

pub fn main() {
    let rodadas = 1000000000;
    let mut casos_favoraveis = 0;

    for _ in 1..rodadas {
        let mut ordem = criar_lista();

        while ordem.iter().filter(|&n| *n == ordem[ordem.len() - 2]).count() == 2 {
            ordem = criar_lista()
        }

        if ordem[0] == 0 && ordem[ordem.len() - 1] == 1 || ordem[ordem.len()  - 1] == 0 && ordem[0] == 1 {
            casos_favoraveis += 1
        }
    }
    println!("{}", casos_favoraveis)
}

fn criar_lista() -> Vec<i32> {
    let mut ordem: Vec<i32> = Vec::new();
    let mut lista = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut rng = rand::thread_rng();

    let mut num = rng.gen_range(0..lista.len());
    let mut a = lista[num];

    ordem.push(a);

    loop {
        num = rng.gen_range(0..lista.len());
        let mut b = lista[num];

        if lista.len() == 1 {
            ordem.push(lista[0]);
            break
        }

        while a == b {
            num = rng.gen_range(0..lista.len());
            b = lista[num];
        }

        a = b;

        if ordem.iter().filter(|&n| *n == ordem[ordem.len() - 1]).count() == 2 {
            ordem.push(b);

        } else {
            lista.remove(num);
            ordem.push(b);
        }
    }
    return ordem;
}
