from random import randint

NUMERO=5
RANDOMNUMERO=randint(0, 10)
if RANDOMNUMERO == NUMERO:
    print('Acertou!')
else:
    if NUMERO > RANDOMNUMERO:
        print('Chutou Alto!')
    else:
        print('Chutou Baixo!')
