# Time Complexity
Time complexity refers to the amount of time an algorithm takes to complete as a function of the size of the input data.

**Common Time Complexities**:
   - *O(1) - Constant Time*:
    The time required by the algorithm is constant, regardless of the input size.
    Example: Accessing an element in an array by index (e.g., arr[5]).

   - *O(log n) - Logarithmic Time*:
    The time required grows logarithmically with the input size. This often occurs in algorithms that repeatedly divide the problem in half.
    Example: *Binary search on a sorted array.*

   - *O(n) - Linear Time*:
    The time required grows linearly with the input size. The algorithm needs to process each element at least once.
    Example: A simple loop that goes through an array.

   - *O(n log n) - Linearithmic Time*:
    The time required is proportional to n multiplied by log n. This is common in efficient sorting algorithms.
    Example: Merge sort, Quick sort.

   - *O(n^2) - Quadratic Time*:
    The time required grows quadratically with the input size. Typically occurs in algorithms with nested loops.
    Example: A double loop checking all pairs of elements in an array.

   - *O(2^n) - Exponential Time*:
    The time required doubles with each additional element in the input. Usually associated with brute-force algorithms.
    Example: Recursive algorithms that solve a problem by splitting it into subproblems (e.g., the naive solution to the Traveling Salesman Problem).

   - *O(n!) - Factorial Time*:
    The time required grows factorially with the input size. This is usually the case with permutations or combinations of all elements.
    Example: Brute-force algorithms generating all permutations of a set.
    
##  Space Complexity.

Space complexity refers to the amount of memory space required by the algorithm as a function of the input size.

*Common Space Complexities*:
    - O(1) - Constant Space:
    The algorithm uses a fixed amount of memory, regardless of the input size.
    Example: Swapping two variables.
   -  O(n) - Linear Space:
    The memory required grows linearly with the input size.
    Example: Storing an array or list of n elements.
    
   - O(log n) - Logarithmic Space:
    The memory required grows logarithmically with the input size. This is less common but can occur in recursive algorithms where the depth of recursion is logarithmic.
    Example: Recursive binary search.

   - O(n^2) - Quadratic Space:
    The memory required grows quadratically with the input size.
    Example: A 2D matrix of size n x n.

## Understanding and Analyzing Complexities.

**Example 1: Linear Search (Finding an Element in an Array)**.

Algorithm: Iterate through each element in the array to find a target value.
Time Complexity: O(n) because in the worst case, you might have to check every element.
Space Complexity: O(1) because you only need a fixed amount of extra space (e.g., a variable to hold the index or the result).

**Example 2: Merge Sort (Sorting an Array).**
Algorithm: Recursively split the array into two halves, sort each half, and then merge the sorted halves.
Time Complexity: O(n log n) because splitting takes log n steps and merging takes linear time.
Space Complexity: O(n) because you need extra space to hold the merged results.

**Example 3: HashMap Lookup**
Algorithm: Insert key-value pairs into a hash map and retrieve values by keys.
Time Complexity:
Insertion: O(1) on average, but can degrade to O(n) in the worst case if many keys hash to the same index.
Lookup: O(1) on average for the same reasons.
Space Complexity: O(n) because you need space to store the key-value pairs.

## Big-O Notation in Practice.
When analyzing an algorithm:
   - Worst-Case Scenario: Consider the maximum amount of work the algorithm might have to do.
   -  Input Size (n): How does the algorithm scale as the input size increases?
   - Focus on Growth: Big-O notation ignores constant factors and lower-order terms, focusing on how the algorithm scales with large inputs.



### Tips for Calculating Time and Space Complexities

 - Identify Loops: A single loop over n elements is O(n). Nested loops are typically O(n^2), etc.

 - Identify Recursions: Recursive calls often lead to O(log n), O(n log n), or worse, depending on the number of recursive calls.
- Track Memory Usage: Keep an eye on data structures like arrays, hash maps, and trees, which take up space proportional to the number of elements.

### Summary:
Time Complexity tells you how the execution time grows as the input size grows.
Space Complexity tells you how the memory usage grows as the input size grows.
Focus on the worst-case scenario when evaluating algorithms.
The goal is to design algorithms that are as efficient as possible, both in terms of time and space.

*Real-World Implications: Understanding time and space complexities helps predict performance, scalability, and  memory usage, which is crucial for optimizing and choosing algorithms for real-world applications*.