#!/usr/bin/env python
""" script that generates several possible orders and counts the favorable events """

from random import choice

ROUNDS           = 1000000
PERSONS          = 10
FAVORABLE_EVENTS = 0

def create_order():
    """ Returns the order in which the draw was made """
    order = []
    list_of_persons = list(range(PERSONS))
    person1 = choice(list_of_persons)
    order.append(person1)

    while True:
        person2 = choice(list_of_persons)

        if len(list_of_persons) == 1:
            order.append(list_of_persons[0])
            break

        while person1 == person2:
            person2 = choice(list_of_persons)

        person1 = person2

        if order.count(order[-1]) == 2:
            order.append(person2)
        else:
            list_of_persons.remove(person2)
            order.append(person2)

    return order

for c in range(ROUNDS):
    order = create_order()

    # check if the list is valid
    while order.count(order[-2]) == 2:
        order = create_order()

    if order[0] == 0 and order[PERSONS -1] == 1 or order[PERSONS -1] == 0 and order[0] == 1:
        FAVORABLE_EVENTS += 1

print(FAVORABLE_EVENTS)
