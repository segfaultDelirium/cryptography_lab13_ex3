// ocena koncowa // dodac punkty podzielic przez liczbe punktow / 20 * 100 procent i to jest ocena

/*

jesli n jest liczba zlozona czyli ma dzielniki
to n ma dzielnik pierwszy p <= podloga(sqrt(n))

n = n1 * n2 gdzie 1 < n1 < n i 1 < n2 < n
wystarczy znalezc jedna z tych liczb

w algorytmie pollarda
d to nietrywialny dzielnik n

B jest ustalane heurystycznie

B powinno być mniejsze niż sqrt(n) żeby algorytm działał szybko

przyklad n = 299 = 13 * 23
j   a   a^j mod n   d
2   2   4           1
3   4   64          1
4   64  27          13

przyjmijmy B rowne 4

kolejne a dostaje 27

jezeli wybierzemy za male B to nie otrzymamy wyniku,
jezeli wybierzemy za duze B to będziemy dlugo czekac


algorytm RHO

zalozmy ze p jest najmniejszym dzielnikiem pierwszym n

zalozmy ze istnieja dwie liczby calkowite ktore maja te same reszty mod p ale są innymi liczbami
wtedy p <= NWD(x - x', n) < n
wybieramy losowy podzbior x a potem obliczamy powyzej dla wszystkich roznych wartosci x, x'
trzeba znalezc takie x ze trafiamy w cykl graniczny czyli krecimy sie w kolko

jezeli |X| to okolo 1.17 * sqrt(p) to jest 50% ze bedzie co najmniej jedna
kolizja (czyli wrocenie do punktu w ktorym juz sie bylo)
wtedy obliczamy NWD(x-x', n)

f(x) = x^2 + 1
x = f(x) mod p zachowuje sie jak odwzorowanie losowe

przyjmijmy ze xi przystaje do xj mod p (czyli xi oraz xj maja takie same reszty po redukcji modulo p)
f(xi) === f(xj) mod p (f(x) zatem tez przystaje

xi+1 = f(xi) mod n

zatem iterujemy po i oraz j funkcje f(x) = x^2 + 1

przyklad n = 7171 = 71 & 101 f(x) = x^2 + 1, x1 = 1

ciag xi zaczyna sie:
1, 2, 5, 26, 677

5 = 2 * 2 + 1
26 = 5 * 5 + 1

pierwsza kolizja to x7 mod 71 = x18 mod 71 = 58
cykl sklada sie z ogona 7 i cyklu 11

aby przyspieszyc szukanie kolizji przyjmujemy j = 2*i

zatem w rozkladzie pierwsza kolizja pojawia sie dla i = 7 i j = 18

film rainman, potrafi liczyc wykalaczki

*/

fn f(x: u128) -> u128 {
    x * x + 1
}

fn rho(n: u128, x1: u128) -> (Option<u128>, u128) {
    let mut x = x1;
    let mut xprim = modulo_euclid(f(x) as i128, n as i128) as u128;
    let mut p = NWD((x as i128 - xprim as i128).abs() as u128, n);
    let mut iteration_counter = 0;
    while p == 1 {
        iteration_counter += 1;
        x = modulo_euclid(f(x) as i128, n as i128) as u128;
        xprim = modulo_euclid(f(xprim) as i128, n as i128) as u128;
        xprim = modulo_euclid(f(xprim) as i128, n as i128) as u128;
        p = NWD((x as i128 - xprim as i128).abs() as u128, n);
    }
    if p == n {
        return (None, iteration_counter);
    }

    return (Some(p), iteration_counter);
}

// fn pollard(n: i32, B: i32) -> Option<u128>{
//     let mut a: u128 = 2;
//     for j in (2..=B){
//         a = modulo_euclid(a.pow(j as u32) as i128, n as i128) as u128;
//     }
//     let d = NWD(a - 1, n as u128);
//
//     if(d > 1 && d < n as u128){
//         return Some(d);
//     }
//     return None;
// }

fn find_dividers(n: u128){
    println!("n = {n}");
    let (p, iteration_count) = rho(n, 1);
    match p {
        Some(p_value) => {
            println!("found p = {p_value} with iteration count: {iteration_count}");
            let other_p = n / p_value;
            println!("other p = {other_p}");
        }
        None => {
            println!("failed to find p with iteration count: {iteration_count}");
        }
    }
    println!();
}

fn main() {
    println!("rho pollard");
    find_dividers(262063);
    find_dividers(9420457);
    find_dividers(181937053);
}



fn NWD(j: u128, k: u128) -> u128 {
    if j == 0 {
        return k
    }
    let r = k % j;
    return NWD(r, j);
}

fn modulo_euclid(j: i128, k: i128) -> i128 {
    let res =  j % k;
    if res < 0 {return res + k} else {return res}
}

// fn abs(a: u128, b: u128) -> u128 {
//
// }

