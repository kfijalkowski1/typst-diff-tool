# How does our comparing work?

When comparing two AST trees of types files we have a few possibilities of coloring the 
compared trees:

If section was **deleted** it will be red

![img_1.png](img_1.png)

If section this section was **added** it will be green

![img_2.png](img_2.png)

If section this section was **moved** it will be yellow

![img_3.png](img_3.png)

If section was **modified** it will be shown like that:

![img.png](img.png)


## How does algorithm work:
example based on fictional nodes:
```html

nodes1 = ["BOX0", "BOX1", "BOX2", "BOX3", "BOX99", "BOX4", "BOX5"]
nodes2 = ["BOX1", "BOX5", "BOX2", "BOX55", "BOX4", "BOX3", "BOX69"]



result:
BOX0 -- deleted
BOX1 -- same
BOX5 -- moved
BOX2 -- same
BOX55 -- added
BOX4 -- moved
BOX3 -- same
('BOX4', 'BOX5') -- modified

--- EXAMPLE 2 ---

list1 = ["BOX0", "BOX1", "BOX2", "BOX3", "BOX99", "BOX4", "BOX5"]
list2 = ["BOX0", "BOX1", "BOX2", "BOX3", "BOX98", "BOX4", "BOX5"]

result:
BOX0 -- same
BOX1 -- same
BOX2 -- same
BOX3 -- same
('BOX98', 'BOX99') -- modified
BOX4 -- same
BOX5 -- same

-- EXAMPLE 3 --
list1 = ["BOX0", "BOX1", "BOX2", "BOX3", "BOX99", "BOX4", "BOX5"]
list2 = ["BOX1", "BOX2", "BOX3", "BOX98", "BOX4", "BOX5", "BOX0"]

result:
BOX1 -- moved
BOX2 -- moved
BOX3 -- moved
BOX98 -- added
BOX4 -- moved
BOX5 -- moved
BOX0 -- same
BOX99 -- deleted
```

**Python code** (less complex for easier understainsing of algorythm)
```python
def compare_nodes(list1, list2):

    added_in1st_nodes = [node for node in list1 if node not in list2]
    added_in2nd_nodes = [node for node in list2 if node not in list1]


    result = []
    resultFlags = []

    len1, len2 = len(list1), len(list2)
    i, j = 0, 0

    while i < len1 or j < len2:
        if i < len1 and list1[i] in result:
            i += 1
        elif i < len1 and j < len2 and list1[i] in added_in1st_nodes and list2[j] in added_in2nd_nodes:
            result.append((list2[i], list1[j]))
            resultFlags.append('modified')
            i += 1
            j += 1
        elif i < len1 and list1[i] in added_in1st_nodes:
            result.append(list1[i])
            resultFlags.append('deleted')
            i += 1
        elif j < len2 and list2[j] in added_in2nd_nodes:
            result.append(list2[j])
            resultFlags.append('added')
            j += 1
        elif i < len1 and j < len2 and list1[i] == list2[j]:
            result.append(list1[i])
            resultFlags.append('same')
            i += 1
            j += 1
        elif i < len1 and j < len2 and list1[i] != list2[j]:
            result.append(list2[j])
            resultFlags.append('moved')
            j += 1

    return result, resultFlags


```
_Rust code is in source code_


##### What will not be supported:
- If the paragraph will be moved and modified at the same time it will be shown as deleted and added in a different place
- If there are two identical blocks in the typ file they will not be annotated correctly (if moved or modified) 