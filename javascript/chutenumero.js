const numero = 5
const randomnumero = Math.floor(Math.random() * 10)
if(randomnumero == numero) {
    console.log("acertou!");
}
if(randomnumero > numero) {
    console.log("Chutou baixo!")
} else {
    console.log("Chutou alto!")
}