namespace Day05;

public class Mapper
{
    public Mapper(string name, string section)
    {
        Name = name;
        var lines = section.Split(Environment.NewLine);
        Ranges = lines.Skip(1).Select(line => new Range(line)).ToList();
    }
    
    public string Name { get; set; }
    public List<Range> Ranges { get; set; }

    public double Map(double x)
    {
        var mapper = Ranges.FirstOrDefault(r => r.IsMatch(x));

        if (mapper == null)
        {
            // Console.Write($" -> {Name} {x} (fallback)");
            return x;
        }

        var mapped = mapper.Map(x);
        // Console.Write($" -> {Name} {mapped}");
        return mapped;
    }
}