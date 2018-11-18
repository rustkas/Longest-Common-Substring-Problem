import java.util.Arrays;

public class Main2 {

	private static String findString(String string) {
		String interStr = "";
		String maxStr = "";
		if (null == string || 2 >= string.length()) {
			return string;
		}

		char str[] = string.toCharArray();
		int start = 0;
		int i = 1;
		while (i < str.length && str[i] == str[start]) {
			i++;
			if (i == str.length) {
				return String.copyValueOf(str);
			}

		}

		maxStr = String.valueOf(Arrays.copyOfRange(str, start, i));

		char chars[] = new char[] { str[start], str[i] };
		int lastGroupStart = 0;
		while (i < str.length) {
			if (str[i] == chars[0] || str[i] == chars[1]) {
				if (str[i] != str[i - 1]) {
					lastGroupStart = i;
					interStr = String.valueOf(Arrays.copyOfRange(str, start,
							i + 1));
					if (interStr.length() > maxStr.length()) {
						maxStr = interStr;
					}
				}
			} else {
				interStr = String.valueOf(Arrays.copyOfRange(str, start, i));

				if (interStr.length() > maxStr.length()) {
					maxStr = interStr;
				}

				start = lastGroupStart;
				lastGroupStart = i;
				chars[0] = str[start];
				chars[1] = str[lastGroupStart];
			}
			i++;
		}
		return maxStr;

	}

	public static void main(String[] args) throws Exception {

		// String str =
		// "wwwwwrrrrrrrrrrrrwwwwwwrrrrrraabadefghaabbaagadDADAAADAADDABBBBBBBDDDDDDDDBBBBBBBBBBBBBB";
		// String str = "";
		// String str = "ab";
		String str = "ABBA";
		String result = findString(str);

		// if(!"wwwwwrrrrrrrrrrrrwwwwwwrrrrrr".equals(result)){
		// System.err.println(result);
		// throw new Exception();
		// }
		System.out.println(result);

	}

}
