
import java.util.HashMap;
import java.util.Map;

public class DialingCodes {

    private final Map<Integer, String> codes = new HashMap<>();

    public Map<Integer, String> getCodes() {
        return codes;
    }

    public void setDialingCode(Integer code, String country) {
        codes.put(code, country);
    }

    public String getCountry(Integer code) {
       return codes.get(code);
    }

    public void addNewDialingCode(Integer code, String country) {
        if(codes.containsKey(code) || codes.containsValue(country)){
            return;
        }
        codes.put(code, country);
    }

    public Integer findDialingCode(String country) {
        for (Map.Entry<Integer, String> kv: codes.entrySet()){
            if (kv.getValue().equals(country)){
                return kv.getKey();
            }
        }
        return null;
    }

    public void updateCountryDialingCode(Integer code, String country) {
        for (Map.Entry<Integer, String> kv : codes.entrySet()) {
            if (kv.getValue().equals(country)) {
                codes.remove(kv.getKey());
                codes.put(code, country);
                return;
            }
        }
    }
}
