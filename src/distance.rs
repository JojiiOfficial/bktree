pub fn levenshtein_distance<S: AsRef<str>>(a: &S, b: &S) -> usize {
    let a = a.as_ref();
    let b = b.as_ref();

    if a == b {
        return 0;
    }

    let a_len = a.chars().count();
    let b_len = b.chars().count();

    if a_len == 0 {
        return b_len;
    }

    if b_len == 0 {
        return a_len;
    }

    let mut res = 0;
    let mut cache: Vec<usize> = (1..).take(a_len).collect();
    let mut a_dist;
    let mut b_dist;

    for (ib, cb) in b.chars().enumerate() {
        res = ib;
        a_dist = ib;
        for (ia, ca) in a.chars().enumerate() {
            b_dist = if ca == cb { a_dist } else { a_dist + 1 };
            a_dist = cache[ia];

            res = if a_dist > res {
                if b_dist > res {
                    res + 1
                } else {
                    b_dist
                }
            } else if b_dist > a_dist {
                a_dist + 1
            } else {
                b_dist
            };

            cache[ia] = res;
        }
    }

    res
}
