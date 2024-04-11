using System;
using System.Collections.Generic;
using System.IO;

namespace cshtupdater
{
    internal class Program
    {
        
        public static void Main(string[] args)
        {
            Dictionary<string, string> HashFiles = new Dictionary<string, string>
            {
                { "binentries", "https://github.com/CommunityDragon/Data/blob/master/hashes/lol/hashes.binentries.txt" },
                { "binfields", "https://github.com/CommunityDragon/Data/blob/master/hashes/lol/hashes.binfields.txt" },
                { "binhashes", "https://github.com/CommunityDragon/Data/blob/master/hashes/lol/hashes.binhashes.txt" },
                { "bintypes", "https://github.com/CommunityDragon/Data/blob/master/hashes/lol/hashes.bintypes.txt" },
                { "game0", "https://github.com/CommunityDragon/Data/blob/master/hashes/lol/hashes.game.txt.0" },
                { "game1", "https://github.com/CommunityDragon/Data/blob/master/hashes/lol/hashes.game.txt.1" },
                { "lcu", "https://github.com/CommunityDragon/Data/blob/master/hashes/lol/hashes.lcu.txt" },
                { "rst", "https://github.com/CommunityDragon/Data/blob/master/hashes/lol/hashes.rst.txt" }
            };
            
            string path = Directory.GetCurrentDirectory();

            string config = path + "\\config.txt";
            if (!File.Exists(config))
            {
                using (var fw = new StreamWriter(config))
                {
                    fw.WriteLine("2000-01-01");
                }
            }

            using (var fr = new StreamReader(config))
            {
                string line;
                while ((line = fr.ReadLine()) != null)
                {
                    DateTime lastUpdate = DateTime.Parse(line);
                    if (lastUpdate.AddDays(7) < DateTime.Now)
                    {
                        Console.WriteLine("Updating...");
                        foreach (var hash in HashFiles)
                        {
                            Console.WriteLine("Downloading {0}...", hash.Key);
                            using (var client = new System.Net.WebClient())
                            {
                                client.DownloadFile(hash.Value, path + "\\hashes." + hash.Key + ".txt");
                            }
                        }
                    }
                    else
                    {
                        Console.WriteLine("Not updating...");
                    }
                }
            
            }
            using (var fw = new StreamWriter(config))
            {
                fw.WriteLine(DateTime.Now.ToString("yyyy-MM-dd"));
            }
        }
    }
}