import java.math.BigInteger;

public class Factorial {

    public static void factorialfun(int n){ 
        BigInteger factorial = BigInteger.valueOf(1);
        for (int i = 1; i <= n; i++) {
            factorial = factorial.multiply(BigInteger.valueOf(i));
        }
    }

    public static void main(String[] args) {
        long tInit1 = System.nanoTime();
        factorialfun(10000);
        System.out.println("Java, N = 10000, time:" + (System.nanoTime() - tInit1));
        
        long tInit2 = System.nanoTime();
        factorialfun(20000);
        System.out.println("Java, N = 20000, time:" + (System.nanoTime() - tInit2));

        long tInit3 = System.nanoTime();
        factorialfun(30000);
        System.out.println("Java, N = 30000, time:" + (System.nanoTime() - tInit3));

    }
}