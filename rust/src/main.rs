use rand::Rng;

fn main() {
    let rounds = 1000000000;
    let mut favorable_cases = 0;

    for _ in 1..rounds {
        let mut order = create_order();

        while !order_is_valid(&order) {
            order = create_order()
        };

        if order[0] == 0 && order[order.len() - 1] == 1
            || order[order.len() - 1] == 0 && order[0] == 1
        {
            favorable_cases += 1
        }
    }
    println!("{}", favorable_cases)
}

fn order_is_valid(order: &Vec<i32>) -> bool {
    let mut isvalid = true;

    if order
        .iter()
        .filter(|&n| *n == order[order.len() - 2])
        .count()
        == 2
    {
        isvalid = false;
    }
    isvalid
}

fn choice(list: &Vec<i32>) -> i32 {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..list.len());
    list[num]
}

fn create_order() -> Vec<i32> {
    let mut order = Vec::new();
    let mut list_of_persons = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut person1 = choice(&list_of_persons);
    order.push(person1);

    loop {
        let mut person2 = choice(&list_of_persons);

        if list_of_persons.len() == 1 {
            order.push(list_of_persons[0]);
            break;
        }

        while person1 == person2 {
            person2 = choice(&list_of_persons)
        }

        person1 = person2;

        if order
            .iter()
            .filter(|&n| *n == order[order.len() - 1])
            .count()
            == 2
        {
            order.push(person2);
        } else {
            let index = list_of_persons.iter().position(|&r| r == person2).unwrap();
            list_of_persons.remove(index);
            order.push(person2);
        }
    }
    order
}
