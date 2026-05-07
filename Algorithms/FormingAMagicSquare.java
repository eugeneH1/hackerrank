import java.io.*;
import java.math.*;
import java.security.*;
import java.text.*;
import java.util.*;
import java.util.concurrent.*;
import java.util.function.*;
import java.util.regex.*;
import java.util.stream.*;
import static java.util.stream.Collectors.joining;
import static java.util.stream.Collectors.toList;

class Result {

    static List<Integer> findCheapestCorner(List<List<Integer>> square) {

        int [][] corners = {{0, 0}, {0, 2}, {2, 2}, {2, 0}};
        int [] magicOne = {8, 4, 2, 6};
        int [] magicTwo = {8, 6, 2, 4};
        List<Integer> costs = new ArrayList<Integer>();

        for (int i = 0; i < 8; i++) {
        int cost = 0;

        for (int j = 0; j < 4; j++) {
            int nextMagicNumber = (i + j) % 4;
            int [] corner = corners[j];

            int x = corner[0];
            int y = corner[1];

            if(i < 4) {
            cost += Math.abs(square.get(x).get(y) - magicOne[nextMagicNumber]);
            } else {
            cost += Math.abs(square.get(x).get(y) - magicTwo[nextMagicNumber]);
            }
        }

        costs.add(cost);
        }
        return costs;
    }

  static List<Integer> findCheapestMiddle(List<List<Integer>> square) {
    
    int[][] mids = {{0, 1}, {1, 2}, {2, 1}, {1, 0}};
    int [] magicOne = {3, 9, 7, 1};
    int [] magicTwo = {1, 7, 9, 3};
    List<Integer> costs = new ArrayList<>();

    for(int i = 0; i < 8; i++) {
      int cost = 0;
      for(int j = 0; j < 4; j++) {
        int nextMagicNumber = (i + j) % 4;
        int[] mid = mids[j];

        int x = mid[0];
        int y = mid[1];

        if(i < 4) {
          cost += Math.abs(square.get(x).get(y) - magicOne[nextMagicNumber]);
        } else {
          cost += Math.abs(square.get(x).get(y) - magicTwo[nextMagicNumber]);
        }
      }

      costs.add(cost);
    }
    return costs;
  }




    public static int formingMagicSquare(List<List<Integer>> s) {
        List<Integer> midCosts = findCheapestMiddle(s);

        List<Integer> cornerCosts = findCheapestCorner(s);

        int middleCost =  Math.abs(s.get(1).get(1) - 5);

        return IntStream.range(0, midCosts.size())
                .map(i -> midCosts.get(i) + cornerCosts.get(i))
                .min()
                .getAsInt() + middleCost;

    }

}

public class Solution {
    public static void main(String[] args) throws IOException {
        BufferedReader bufferedReader = new BufferedReader(new InputStreamReader(System.in));
        BufferedWriter bufferedWriter = new BufferedWriter(new FileWriter(System.getenv("OUTPUT_PATH")));

        List<List<Integer>> s = new ArrayList<>();

        IntStream.range(0, 3).forEach(i -> {
            try {
                s.add(
                    Stream.of(bufferedReader.readLine().replaceAll("\\s+$", "").split(" "))
                        .map(Integer::parseInt)
                        .collect(toList())
                );
            } catch (IOException ex) {
                throw new RuntimeException(ex);
            }
        });

        int result = Result.formingMagicSquare(s);

        bufferedWriter.write(String.valueOf(result));
        bufferedWriter.newLine();

        bufferedReader.close();
        bufferedWriter.close();
    }
}

