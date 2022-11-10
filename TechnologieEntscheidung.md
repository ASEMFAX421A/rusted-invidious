Technologieentscheidung 1:

Ausgangssituation:
Für unser Projekt benötigen wir ein Tool, welches die von Usern produzierten Ereignisse entgegennimmt, verarbeitet und abspeichert. Da es hierfür genügend freie und kostenlose Anwendungen gibt, haben wir uns dazu entschlossen, uns einer von Ihnen zu bedienen. Unsere Entscheidung fiel hierbei auf Apache Kafka.

Über Kafka:
Apache Kafka ist eine Open-Source-Plattform für verteiltes Ereignis-Streaming. Das daraus resultierende System ist ein verteiltes System, das aus Servern und Clients besteht, die über ein leistungsstarkes TCP-Netzwerkprotokoll kommunizieren. Es kann auf Bare-Metall-Hardware, virtuellen Maschinen und Containern in On-premise und Cloud-Umgebungen implementiert werden.

Kafka im Vergleich:
Alternativsystem zu Kafka sind z.B. RabbitMQ und ActiveMQ. Kafka hat gegenüber diesen beiden Systemen jedoch den Vorteil, dass es die Messaging-Protokolle: Publish/Subscribe, Punkt-zu-Punkt und garantierte Bestellung unterstützt.
Von diesen Protokollen verwenden wir Publish/Subscribe und Garantierte Bestellung. ActiveMQ unterstützt die garantierte Bestellung nicht und konnte daher nicht verwendet werden. Bei RabbitMQ ist es ähnlich. Hier wird Publish/Subscribe nicht unterstützt.

Vor- und Nachteile von Kafka:
Neben dem Unterstützen von benötigten Protokollen hat Kafka weitere Vorteile.
Einige von ihnen sind:
• Scability
• Hohe Performance
• Ausfallsicherheit
• Zukunftssicherheit
• Geringe Latenzen
• Freie Open-Source-Software
• Ausgereiftes Ökosystem

Die Nachteile von Kafka sind:
• Hoher Ressourcen verbrauch
• Schwierig aufzusetzen

Anwendungsbeispiel:
Ein konkretes Anwendungsbeispiel für Kafka ist folgendes: Auf einem YT Kanal wird ein neues Video hochgeladen. Als Reaktion sendet ein Microservice ein Push-Event an Kafka. Kafka reagiert auf das Event und informiert über einen weiteren Microservice alle, die über das Event informiert werden wollen (Abonnenten). Außerdem fügt dieser Service der Datenbank einen neuen Eintrag hinzu. Kafka speichert das Event für eine gewisse Zeit. Auf diese Weise gehen die Daten nicht verloren, selbst wenn der Microservice offline ist.