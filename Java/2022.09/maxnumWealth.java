import java.util.Arrays;

/**
 * @author goumang
 * @description
 * @date 2022/9/9 18:23
 */
public class maxnumWealth {

    public int maximumWealth(int[][] accounts) {
        return Arrays.stream(accounts)
                .map(acs -> Arrays.stream(acs).sum())
                .max((x, y) -> x.compareTo(y))
                .get();
    }
}
