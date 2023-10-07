<?php
$numero = 5;
$randomnumero = rand(0, 10);
if($numero == $randomnumero) {
    print('Acertou!');
} else {
    if($numero > $randomnumero) {
        print('Chutou alto!');
    } else {
        print('Chutou baixo!');
    }
}

?>