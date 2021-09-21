#!/usr/bin/env python

from random import choice

rodadas             = int(input(''))
pessoas             = int(input(''))
casos_favoraveis    = 0

def criar_lista():
    ordem = []
    lista = list(range(pessoas))
    a = choice(lista)
    ordem.append(a)

    while True:
        b = choice(lista)

        if len(lista) == 1:
            ordem.append(lista[0])
            break

        while a == b:
            b = choice(lista)

        a = b

        if ordem.count(ordem[-1]) == 2:
            ordem.append(b)
        else:
            lista.remove(b)
            ordem.append(b)

    return ordem

for c in range(rodadas):
    ordem = criar_lista()

    while ordem.count(ordem[-2]) == 2:
        # verifica se a lista é valida
        ordem = criar_lista()

    if ordem[0] == 0 and ordem[pessoas -1] == 1 or ordem[pessoas -1] == 0 and ordem[0] == 1:
        casos_favoraveis += 1

print('casos totais: ', rodadas)
print('casos favoráveis: ',casos_favoraveis)
print('probabilidade: ', casos_favoraveis/rodadas)
