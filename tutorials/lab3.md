
# **Join, Group by, Order by**
## **Join**
Допустим, что мы хотим получить даные из двух таблиц, как нам это сделать? Первое что приходит в голову - SELECT, но это довольно муторно. Для того, чтобы не мучиться был придуман оператор JOIN
Как он работает? Очень просто, т.к. оператор бинарный, то он должен работать с 2 объектами, в нашем случае объектами являются таблицы базы данных. Join получает таблицы которые нужно объединить и критерий для объединения.
Звучит, пожалуй, сложно, поэтому давайте рассмотрим на примере более подробно.
Итак, у нас в примере будет 2 таблицы, а именно: таблица Persons с полями id_person, name, position_ref, а также таблица Positions с полями id, title.
![Иллюстрация](https://github.com/iu5git/Database/tree/main/pictures/м1.png)
Теперь необходимо определиться с тем по какому критерию мы будем объединять таблицы. В данном случае все довольно очевидно, и объединим мы их по единственному подходяещму выражению
`id_pos = position_ref`
Итоговый запрос будет выглядеть следующим образом:

`SELECT id_person, name, id_pos, title 
FROM `persons`
INNER JOIN `positions` ON id_pos = position_ref`

Наиболее внимательные могли заметить приписку INNER перед JOIN. Здесь стоит разобрать все виды объединений. 
Всего их 3: INNER (Такое присоединение покажет нам данные из таблиц, только если условие связывания соблюдается), LEFT/RIGHT (здесь мы увидим все записи из левой/правой таблицы в то время, как поля из правой/левой будут добавлены по возможности)
LEFT и RIGHT JOIN отличаются лишь тем, какая из таблиц будет выведена полностью, так что нет смысла разибрать оба варианта. Ниже представлен пример работы LEFT JOIN.

`
SELECT id_person, name, id_pos, title 
FROM `persons`
LEFT OUTER JOIN `positions` ON id_pos = position_ref`
![Иллюстрация](https://github.com/iu5git/Database/tree/main/pictures/м2.png)

«Левая» таблица persons, содержит строку id_person#3 — «Александр», где указан идентификатор должности, отсутствующей в словаре. Мы увидим все записи из «левой» таблицы, тогда как правая будет присоединена по возможности.
## **Group by**
Для группировки данных в PostgreSQL применяются операторы GROUP BY и HAVING, для использования которых применяется следующий формальный синтаксис:
`SELECT столбцы
FROM таблица
[WHERE условие_фильтрации_строк]
[GROUP BY столбцы_для_группировки]
[HAVING условие_фильтрации_групп]
[ORDER BY столбцы_для_сортировки]`

Для рассмотрения того, как работают данные операторы возьмем уже другую таблицу:

CREATE TABLE Products`
(
  Id SERIAL PRIMARY KEY,
  ProductName VARCHAR(30) NOT NULL,
 Company VARCHAR(20) NOT NULL,
   ProductCount INT DEFAULT 0,
   Price NUMERIC NOT NULL,
   IsDiscounted BOOL
);`

Главное предназначение данных операторов заключается в том, чтобы анализировать группы строк, полученнные в результате отбора. Это применяется, когда мы хотим получить количество товаров определнного типа или посчитать их общий вес.

Давайте, к примеру, посчитаем количество телефонов разных компаний в нашей БД:
![Иллюстрация](https://github.com/iu5git/Database/tree/main/pictures/м3.png)

Важно понимать, что группировать можно по нескольким столбцам одновременно. Однако, если в столбцах будут содержаться NULL значения, то такие записи будут заноситься в отдельную группу.
Кроме того PostgreSQL позволяет использовать 3 дополнительных параметра вместе с Group by. Чтобы их применить необходимо между Group by и столбцами группировки добавить сам параметр, например вот так GROUP BY ROLLUP(Company);
1) ROLLUP. Оператор ROLLUP добавляет суммирующую строку в результирующий набор.
2) CUBE похож на ROLLUP за тем исключением, что CUBE добавляет суммирующие строки для каждой комбинации групп.
3) Оператор GROUPING SETS группирует получемые наборы отдельно:
4) 
`SELECT Company, COUNT(*) AS Models, ProductCount
FROM Products
GROUP BY GROUPING SETS(Company, ProductCount);`

В выражении SELECT производится выборка компаний, количества моделей и количества товаров. То есть мы получаем три категории. Оператор GROUPING SETS производит группировку по двум столбцам - Company и ProductCount. В итоге будет создаваться две группы: 1) компании и количество моделей и 2)количество моделей и количество товаров.

![Иллюстрация](https://github.com/iu5git/Database/tree/main/pictures/м4.png)


## **Having**
Последнее, что мы изучим в данной лабораторной работе, это - HAVING. Довольно часто можно видеть этот оператор в паре с Group by и вот почему: HAVING позволяет проводить фильтрацию групп, полученных в результате группировки.
Например, если я не хочу, чтобы группы, в которых количество телефонов меньше одного, выводились на экран. Для этого я просто добавлю параметр HAVING COUNT(*)> 1
![Иллюстрация](https://github.com/iu5git/Database/tree/main/pictures/м5.png)

Таким образом мы можем задать дополнительные параметры для фильтрации групп, такие как COUNT, SUM и тп.
# **Задание**
1. Применить 3 различных JOIN
1. Сгруппировать данные из таблицы и посчитать кол-во записей (в зависимости от таблицы в бд)
1. Применить GROUPING SETS, CUBE и ROLLUP
1. Вывести группы, с дополнительным параметром HAVING
# **Контрольные вопросы**
1. Что такое join и для чего они нужны? 
1. Какие бывают join и в чем их отличие?
1. Как можно группировать данные?
1. Для чего вообще группируют данные?
1. Какие дополнительные параметры для Group By есть в PostgreSQL? Что делает каждый из них?
1. Для чего используется Having?
1. Какой синтаксис у JOIN, Group By, Having?
