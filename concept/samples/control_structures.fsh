for (it<int> i in [1->9]) {
    print(i.iteration) // "0", "1", "2", ... ,"8"
    print(i.value) // "1", "2", "3", ... ,"9"
}

for (int i in [0->9]) {
    print(i) // "0", "1", "2", ... ,"9"
}

if (true) {
    // valid
// if we're doing 'not', let's do
// 'and' too, it's nicer to type
} else if (true and false) {
    // unreachable branch
} else {
    break
}