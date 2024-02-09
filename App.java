public class App {
  public static int main(String[] args) {
    int[] a = new int[1];
    a[0] = 1;
    return snd(41, 9);
  }

  static int snd(int x, int y) {
    return (int) Math.sqrt(y);
  }
}
