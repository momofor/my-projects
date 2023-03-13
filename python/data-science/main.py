import csv


def average(lst):
    avg = 0
    for element in lst:
        avg += element
    return avg / len(lst)


with open("./test.csv", newline="") as csvfile:
    spamreader = csv.reader(csvfile)
    list_of_elements = []
    for row in spamreader:
        # if type(row[1]) == "number":
        #     print(row[1])
        print(type(row[1]))
    # print(average(list_of_elements))
