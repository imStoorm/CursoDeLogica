$numero = 5;
$res = $numero;
$i = $numero - 1;
while($i > 0) {
    $res = $res * $i;
    $i = $i - 1;
}
print($res);