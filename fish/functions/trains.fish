function trains --description 'UK live train departures from Kelvedon to London Liverpool Street'
    set -l from KEL
    set -l to LST
    set -l count 10

    curl -s "https://huxley2.azurewebsites.net/departures/$from/to/$to/$count" | jq -r '
      .trainServices[] |
      if .isCancelled then
        "\(.std) -> CANCELLED  (\(.cancelReason // "no reason given"))"
      elif .filterLocationCancelled then
        "\(.std) -> does not call at LST  (\(.cancelReason // "diverted"))"
      elif .etd == "On time" then
        "\(.std) -> on time      plat \(.platform // "?")"
      elif .etd == "Delayed" then
        "\(.std) -> DELAYED      plat \(.platform // "?")  (\(.delayReason // "no reason given"))"
      else
        "\(.std) -> exp \(.etd)  plat \(.platform // "?")  (\(.delayReason // "late"))"
      end
    '
end
