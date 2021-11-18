# Gabarito Enem / 2020 / Segundo Dia / Caderno azul 


![imagem da questão](https://s2.glbimg.com/Rb4gHZsZ7VpBjT3jTnvgve0A4Lc=/0x0:344x435/984x0/smart/filters:strip_icc()/i.s3.glbimg.com/v1/AUTH_59edd422c0c84a879bd37670ae4f538a/internal_photos/bs/2021/q/q/txMSXXQAuL0pmfxqjlcA/amigosecreto.png)

 
Amigo secreto é uma brincadeira tradicional nas festas de fim de ano. Um grupo de amigos se reúne e cada um deles sorteia o nome da pessoa que irá presentear. No dia da troca de presentes, uma primeira pessoa presenteia seu amigo secreto. Em seguida, o presenteado revela seu amigo secreto e o presenteia. A brincadeira continua até que todos sejam presenteados, mesmo no caso em que o ciclo se fecha. Dez funcionários de uma empresa, entre eles um casal, participarão de um amigo secreto. A primeira pessoa a revelar será definida por sorteio.

## Qual é a probabilidade de que a primeira pessoa a revelar o seu amigo secreto e a última presenteada sejam as duas pessoas do casal?

Essa Questão foi anulada por não possuir a resposta correta em suas alternativas, então criei esse algoritimo para tentar aproximar a solução.

Em meus testes eu calculei ***10000000*** (dez milhôes) de vezes e o resultado obtido foi de aproximadamente ***0,0182992***

A ideia desse programa é gerar uma lista do tipo:
[8, 5, 7, 1, 8, 9, 3, 4, 2, 6, 0]
onde apresenta a **ordem** em que foi retirado ou seja
o 8 foi a primeira pessoa que foi sorteado aleatoriamente entregou o presente para o 5 que entregou para o 7 ...
assim podemos analisar o primeiro e o ultimo a receber o presente e aproximar de uma possível reposta
obs: vão existir casos por exemplo [1, 2, 1, 6, 4, 3, 8, 5, 0, 7, 9] em que você percebe que o 1 deu o presente para o dois e o dois deu pro 1 logo o 6 foi gerado aleatoriamente
a ideia principal e ter duas listas onde uma esta numerada de 0 até 9 ([0, 1, 2, 3, 4 ,5 , 6, 7, 8, 9])
e a outra está vazia. onde ira ser escolhido um item aleatoriamente e adicionado a segunda.
depois sera selecionado mais outro item.
se esse item for igual ao anterior escolha outro
se o ultimo item foi inserido duas vezes adicione o item na segunda lista
caso contrario adicione o item e remova-o da primeira lista
se a primeira lista tiver apenas 1 item adicione-o  e feche o loop
por ultimo se verificarar se a lista é valida

## English

Secret friend is a traditional game at the end of the year. A group of friends gets together and each one of them draws the name of the person who will give a gift. On the day of the exchange of gifts, a first person presents a gift to their secret friend. Then, the recipient reveals his secret friend and presents him. The game continues until everyone is presented, even if the cycle closes. Ten employees of a company, including a couple, will participate in a secret friend. The first person to reveal will be chosen by drawing lots.
The idea of this program is to generate a list of the type: [8, 5, 7, 1, 8, 9, 3, 4, 2, 6, 0] where it displays the **order** in which it was removed, ie the 8 it was the first randomly drawn person who delivered the gift to the 5 who delivered it to the 7 ... so we can analyze the first and the last to receive the gift and approximate a possible answer. Note: there will be cases for example [1, 2, 1, 6, 4, 3, 8, 5, 0, 7, 9] where you realize that 1 gave the gift to two and two gave pro 1 so 6 was randomly generated the main idea and have two lists where one is numbered from 0 to 9 ([0, 1, 2, 3, 4 ,5 , 6, 7, 8, 9]) and the other is empty. where an item will be chosen at random and added to the second. then another item will be selected. if this item is the same as the previous one choose another one if the last item was entered twice add the item to the second list otherwise add the item and remove it from the first list if the first list has only 1 item add it and close the loop finally check if the list is valid
(Google Translate)

## What is the probability that the first person to reveal your secret friend and the last person presented are the two people in the couple?
