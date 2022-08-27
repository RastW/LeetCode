public class Ransom {


    public static boolean canConstruct(String ransomNote, String magazine) {
        int length = ransomNote.length();
        if (length > magazine.length()) {
            return false;
        }

        int[] cut = new int[26];

        for (char c : magazine.toCharArray()) {
            System.out.println(c - 'a');
            cut[c - 'a'] ++;
        }

        for (char c : ransomNote.toCharArray()) {
            cut[c - 'a'] --;
            if (cut[c - 'a'] < 0) {
                return false;
            }
        }

        return true;
    }
}

